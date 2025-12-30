// Generated macro for impl_147 (impl)
macro_rules! Depcrate_num_traitsimpl_147 {
() => {
// Module: crate::num_traits
// Provides: {"impl_147"}
// Dependencies: {}
impl NumCast for f16 { # [inline] fn from < T : ToPrimitive > (n : T) -> Option < Self > { n . to_f32 () . map (Self :: from_f32) } }
};
}
