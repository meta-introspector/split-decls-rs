// Generated macro for impl_77 (impl)
macro_rules! Depcrate_bridge_fxhashimpl_77 {
() => {
// Module: crate::bridge::fxhash
// Provides: {"impl_77"}
// Dependencies: {}
impl FxHasher { # [inline] fn add_to_hash (& mut self , i : usize) { self . hash = self . hash . rotate_left (5) . bitxor (i) . wrapping_mul (K) ; } }
};
}
