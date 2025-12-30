// Generated macro for rotr_32_s3 (macro)
macro_rules! Depcrate_x86_64_sse2rotr_32_s3 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"rotr_32_s3"}
// Dependencies: {}
macro_rules ! rotr_32_s3 { ($ name : ident , $ k0 : expr , $ k1 : expr) => { # [inline (always)] fn $ name (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi8 (self . x , _mm_set_epi64x ($ k0 , $ k1)) }) } } ; }
};
}
