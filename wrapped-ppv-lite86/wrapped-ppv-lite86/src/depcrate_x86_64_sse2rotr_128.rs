// Generated macro for rotr_128 (macro)
macro_rules! Depcrate_x86_64_sse2rotr_128 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"rotr_128"}
// Dependencies: {}
macro_rules ! rotr_128 { ($ name : ident , $ i : expr) => { # [inline (always)] fn $ name (self) -> Self { Self :: new (unsafe { _mm_or_si128 (_mm_srli_si128 (self . x , $ i as i32) , _mm_slli_si128 (self . x , 128 - $ i as i32) ,) }) } } ; }
};
}
