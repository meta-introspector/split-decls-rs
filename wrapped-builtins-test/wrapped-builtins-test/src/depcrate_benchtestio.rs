// Generated macro for TestIO (trait)
macro_rules! Depcrate_benchTestIO {
() => {
// Module: crate::bench
// Provides: {"TestIO"}
// Dependencies: {}
# [doc = " A type used as either an input or output to/from a benchmark function."] pub trait TestIO : Sized { fn make_testvec (len : u32) -> Vec < Self > ; fn check_eq (a : Self , b : Self) -> bool ; }
};
}
