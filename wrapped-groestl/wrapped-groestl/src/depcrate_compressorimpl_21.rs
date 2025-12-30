// Generated macro for impl_21 (impl)
macro_rules! Depcrate_compressorimpl_21 {
() => {
// Module: crate::compressor
// Provides: {"impl_21"}
// Dependencies: {}
impl Map2 for (X4 , X4) { type Output = X4 ; # [inline (always)] fn map < F > (self , mut f : F) -> Self :: Output where F : FnMut (__m128i , __m128i) -> __m128i , { let (a , b) = self ; X4 (f (a . 0 , b . 0) , f (a . 1 , b . 1) , f (a . 2 , b . 2) , f (a . 3 , b . 3)) } }
};
}
