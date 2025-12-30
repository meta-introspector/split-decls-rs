// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { color_eyre :: install () ? ; let terminal = ratatui :: init () ; execute ! (stdout () , EnableMouseCapture) ? ; let app_result = run (terminal) ; ratatui :: restore () ; if let Err (err) = execute ! (stdout () , DisableMouseCapture) { eprintln ! ("Error disabling mouse capture: {err}") ; } app_result }
};
}
