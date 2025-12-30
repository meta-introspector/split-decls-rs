// Generated macro for impl_127 (impl)
macro_rules! Depcrate_debtimpl_127 {
() => {
// Module: crate::debt
// Provides: {"impl_127"}
// Dependencies: {}
impl Debt { # [doc = " The value of pointer `3` should be pretty safe, for two reasons:"] # [doc = ""] # [doc = " * It's an odd number, but the pointers we have are likely aligned at least to the word size,"] # [doc = "   because the data at the end of the `Arc` has the counters."] # [doc = " * It's in the very first page where NULL lives, so it's not mapped."] pub (crate) const NONE : usize = 0b11 ; }
};
}
