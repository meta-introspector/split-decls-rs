// Generated macro for impl_187 (impl)
macro_rules! Depcrate_num_traitsimpl_187 {
() => {
// Module: crate::num_traits
// Provides: {"impl_187"}
// Dependencies: {}
impl NumCast for bf16 { # [inline] fn from < T : ToPrimitive > (n : T) -> Option < Self > { n . to_f32 () . map (Self :: from_f32) } }
};
}
