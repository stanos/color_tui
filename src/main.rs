use std::{io::{self, Write}, thread, time::Duration};
use tui::{text, style::{Style, Color}, backend::{CrosstermBackend, self}, Terminal, terminal, widgets::{Widget, Block, Borders, Paragraph}, layout::{Layout, Constraint, Direction}, text::{Text, Span}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let style = Style::default().fg(Color::Yellow);


    terminal.draw(|f| {
        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        
        let mut text = Text::from("Hello World!");
        text.patch_style(style);
        let text_paragraph = Paragraph::new(text);

        f.render_widget(block, size);
        f.render_widget(text_paragraph, chunks[0]);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
