// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn std :: error :: Error > > { println ! ("🏗️  Incremental Crate Builder") ; let mut builder = IncrementalBuilder :: new () ; builder . scan_output2 () ? ; builder . build_incrementally () ? ; Ok (()) }
};
}
