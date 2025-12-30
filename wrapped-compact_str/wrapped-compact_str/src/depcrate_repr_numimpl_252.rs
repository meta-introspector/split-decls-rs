// Generated macro for impl_252 (impl)
macro_rules! Depcrate_repr_numimpl_252 {
() => {
// Module: crate::repr::num
// Provides: {"impl_252"}
// Dependencies: {}
impl NumChars for isize { fn num_chars (val : isize) -> usize { # [cfg (target_pointer_width = "32")] { i32 :: num_chars (val as i32) } # [cfg (target_pointer_width = "64")] { i64 :: num_chars (val as i64) } } }
};
}
