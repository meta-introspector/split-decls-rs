// Generated macro for rotr_64 (macro)
macro_rules! Depcrate_x86_64_sse2rotr_64 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"rotr_64"}
// Dependencies: {}
macro_rules ! rotr_64 { ($ name : ident , $ i : expr) => { # [inline (always)] fn $ name (self) -> Self { Self :: new (unsafe { _mm_or_si128 (_mm_srli_epi64 (self . x , $ i as i32) , _mm_slli_epi64 (self . x , 64 - $ i as i32) ,) }) } } ; }
};
}
