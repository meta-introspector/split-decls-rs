// Generated macro for impl_251 (impl)
macro_rules! Depcrate_repr_numimpl_251 {
() => {
// Module: crate::repr::num
// Provides: {"impl_251"}
// Dependencies: {}
impl NumChars for usize { fn num_chars (val : usize) -> usize { # [cfg (target_pointer_width = "32")] { u32 :: num_chars (val as u32) } # [cfg (target_pointer_width = "64")] { u64 :: num_chars (val as u64) } } }
};
}
