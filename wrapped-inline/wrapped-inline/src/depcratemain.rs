// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { color_eyre :: install () ? ; let mut terminal = ratatui :: init_with_options (TerminalOptions { viewport : Viewport :: Inline (8) , }) ; let (tx , rx) = mpsc :: channel () ; input_handling (tx . clone ()) ; let workers = workers (tx) ; let mut downloads = downloads () ; for w in & workers { let d = downloads . next (w . id) . unwrap () ; w . tx . send (d) . unwrap () ; } let app_result = run (& mut terminal , workers , downloads , rx) ; ratatui :: restore () ; app_result }
};
}
