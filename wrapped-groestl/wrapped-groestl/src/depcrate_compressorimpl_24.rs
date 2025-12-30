// Generated macro for impl_24 (impl)
macro_rules! Depcrate_compressorimpl_24 {
() => {
// Module: crate::compressor
// Provides: {"impl_24"}
// Dependencies: {}
impl Map2 for (X8 , X8) { type Output = X8 ; # [inline (always)] fn map < F > (self , mut f : F) -> Self :: Output where F : FnMut (__m128i , __m128i) -> __m128i , { let (a , b) = self ; X8 (f (a . 0 , b . 0) , f (a . 1 , b . 1) , f (a . 2 , b . 2) , f (a . 3 , b . 3) , f (a . 4 , b . 4) , f (a . 5 , b . 5) , f (a . 6 , b . 6) , f (a . 7 , b . 7) ,) } }
};
}
