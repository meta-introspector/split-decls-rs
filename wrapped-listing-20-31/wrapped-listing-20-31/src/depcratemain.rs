// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { enum Status { Value (u32) , Stop , } let list_of_statuses : Vec < Status > = (0u32 .. 20) . map (Status :: Value) . collect () ; }
};
}
