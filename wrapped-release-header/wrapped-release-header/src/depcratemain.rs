// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> color_eyre :: Result < () > { color_eyre :: install () ? ; let viewport = Viewport :: Fixed (Rect :: new (0 , 0 , 68 , 16)) ; let terminal = ratatui :: init_with_options (TerminalOptions { viewport }) ; execute ! (stdout () , EnterAlternateScreen) . expect ("failed to enter alternate screen") ; let result = run (terminal) ; execute ! (stdout () , LeaveAlternateScreen) . expect ("failed to leave alternate screen") ; ratatui :: restore () ; result }
};
}
