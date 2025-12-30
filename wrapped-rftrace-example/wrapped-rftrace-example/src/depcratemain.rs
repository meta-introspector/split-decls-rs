// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let events = rftrace :: init (2000 , false) ; rftrace :: enable () ; f1 () ; std :: hint :: black_box (()) ; rftrace :: dump_full_uftrace (events , "/root/tracedir" , "rftrace-example") . unwrap () ; }
};
}
