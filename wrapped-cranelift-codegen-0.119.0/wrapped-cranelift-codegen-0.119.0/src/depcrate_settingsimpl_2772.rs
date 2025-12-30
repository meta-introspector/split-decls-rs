// Generated macro for impl_2772 (impl)
macro_rules! Depcrate_settingsimpl_2772 {
() => {
// Module: crate::settings
// Provides: {"impl_2772"}
// Dependencies: {}
impl < 'a > PredicateView < 'a > { # [doc = " Create a new view of a precomputed predicate vector."] # [doc = ""] # [doc = " See the `predicate_view()` method on the various `Flags` types defined for each ISA."] pub fn new (bits : & 'a [u8]) -> Self { PredicateView (bits) } # [doc = " Check a numbered predicate."] pub fn test (self , p : usize) -> bool { self . 0 [p / 8] & (1 << (p % 8)) != 0 } }
};
}
