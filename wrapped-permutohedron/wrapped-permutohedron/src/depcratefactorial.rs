// Generated macro for factorial (function)
macro_rules! Depcratefactorial {
() => {
// Module: crate
// Provides: {"factorial"}
// Dependencies: {}
# [doc = " Compute *n!* (*n* factorial)"] # [must_use] pub fn factorial (n : usize) -> usize { (1 ..= n) . product :: < usize > () }
};
}
