// Generated macro for IndexRange (trait)
macro_rules! Depcrate_rangeIndexRange {
() => {
// Module: crate::range
// Provides: {"IndexRange"}
// Dependencies: {}
# [doc = " **IndexRange** is implemented by Rust's built-in range types, produced"] # [doc = " by range syntax like `..`, `a..`, `..b` or `c..d`."] pub trait IndexRange < T = usize > { # [inline] # [doc = " Start index (inclusive)"] fn start (& self) -> Option < T > { None } # [inline] # [doc = " End index (exclusive)"] fn end (& self) -> Option < T > { None } }
};
}
