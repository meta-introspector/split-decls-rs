// Generated macro for impl_136 (impl)
macro_rules! Depcrate_x86_64_sse2impl_136 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_136"}
// Dependencies: {}
impl < S4 : Copy , NI : Copy > RotateEachWord32 for u64x2_sse2 < NoS3 , S4 , NI > { rotr_64 ! (rotate_each_word_right7 , 7) ; rotr_64 ! (rotate_each_word_right8 , 8) ; rotr_64 ! (rotate_each_word_right11 , 11) ; rotr_64 ! (rotate_each_word_right12 , 12) ; # [inline (always)] fn rotate_each_word_right16 (self) -> Self { Self :: new (swap16_s2 (self . x)) } rotr_64 ! (rotate_each_word_right20 , 20) ; rotr_64 ! (rotate_each_word_right24 , 24) ; rotr_64 ! (rotate_each_word_right25 , 25) ; }
};
}
