// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { color_eyre :: install () ? ; stdout () . execute (EnableMouseCapture) ? ; let terminal = ratatui :: init () ; let app_result = App :: new () . run (terminal) ; ratatui :: restore () ; stdout () . execute (DisableMouseCapture) ? ; app_result }
};
}
