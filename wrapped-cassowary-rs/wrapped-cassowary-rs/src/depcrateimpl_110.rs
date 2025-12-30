// Generated macro for impl_110 (impl)
macro_rules! Depcrateimpl_110 {
() => {
// Module: crate
// Provides: {"impl_110"}
// Dependencies: {}
impl Variable { # [doc = " Produces a new unique variable for use in constraint solving."] pub fn new () -> Variable { Variable (VARIABLE_ID . fetch_add (1 , :: std :: sync :: atomic :: Ordering :: Relaxed)) } }
};
}
