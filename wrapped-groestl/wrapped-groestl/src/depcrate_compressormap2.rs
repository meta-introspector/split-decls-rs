// Generated macro for Map2 (trait)
macro_rules! Depcrate_compressorMap2 {
() => {
// Module: crate::compressor
// Provides: {"Map2"}
// Dependencies: {}
trait Map2 { type Output ; fn map < F > (self , f : F) -> Self :: Output where F : FnMut (__m128i , __m128i) -> __m128i , Self : Sized ; }
};
}
