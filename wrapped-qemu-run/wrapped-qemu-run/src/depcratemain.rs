// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , anyhow :: Error > { notmain () . map (| opt_code | { if let Some (code) = opt_code { process :: exit (code) ; } }) }
};
}
