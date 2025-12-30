// Generated macro for impl_542 (impl)
macro_rules! Depcrate_limitimpl_542 {
() => {
// Module: crate::limit
// Provides: {"impl_542"}
// Dependencies: {}
impl Limit { # [doc = " Create a new limit from a `usize`."] pub fn new (value : usize) -> Self { Limit (value) } # [doc = " Create a new unlimited limit."] pub fn unlimited () -> Self { Limit (usize :: MAX) } # [doc = " Check that `value` is within the limit. Ensures that the same comparisons are used"] # [doc = " throughout the compiler, as mismatches can cause ICEs, see #72540."] # [inline] pub fn value_within_limit (& self , value : usize) -> bool { value <= self . 0 } }
};
}
