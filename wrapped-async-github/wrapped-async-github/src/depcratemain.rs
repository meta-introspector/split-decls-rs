// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [tokio :: main] async fn main () -> Result < () > { color_eyre :: install () ? ; let terminal = ratatui :: init () ; let app_result = App :: default () . run (terminal) . await ; ratatui :: restore () ; app_result }
};
}
