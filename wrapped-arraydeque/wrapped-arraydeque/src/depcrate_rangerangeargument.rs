// Generated macro for RangeArgument (trait)
macro_rules! Depcrate_rangeRangeArgument {
() => {
// Module: crate::range
// Provides: {"RangeArgument"}
// Dependencies: {}
# [doc = " **RangeArgument** is implemented by Rust's built-in range types, produced"] # [doc = " by range syntax like `..`, `a..`, `..b` or `c..d`."] pub trait RangeArgument < T = usize > { # [inline] # [doc = " Start index (inclusive)"] fn start (& self) -> Option < T > { None } # [inline] # [doc = " End index (exclusive)"] fn end (& self) -> Option < T > { None } }
};
}
