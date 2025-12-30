// Generated macro for impl_221 (impl)
macro_rules! Depcrate_x86_64_sse2impl_221 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_221"}
// Dependencies: {}
impl < S3 , S4 , NI > PartialEq for u32x4_sse2 < S3 , S4 , NI > { # [inline (always)] fn eq (& self , rhs : & Self) -> bool { unsafe { eq128_s2 (self . x , rhs . x) } } }
};
}
