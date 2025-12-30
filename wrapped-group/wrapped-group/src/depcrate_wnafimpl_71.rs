// Generated macro for impl_71 (impl)
macro_rules! Depcrate_wnafimpl_71 {
() => {
// Module: crate::wnaf
// Provides: {"impl_71"}
// Dependencies: {}
impl < G : Group , const WINDOW_SIZE : usize > WnafBase < G , WINDOW_SIZE > { # [doc = " Computes a window table for the given base with the specified `WINDOW_SIZE`."] pub fn new (base : G) -> Self { let mut table = vec ! [] ; wnaf_table (& mut table , base , WINDOW_SIZE) ; WnafBase { table } } }
};
}
