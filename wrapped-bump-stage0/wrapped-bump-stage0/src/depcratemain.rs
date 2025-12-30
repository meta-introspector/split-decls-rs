// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Error > { let tool = Tool :: new (std :: env :: args () . nth (1)) ? ; tool . update_stage0_file () ? ; Ok (()) }
};
}
