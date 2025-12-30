// Generated macro for impl_2067 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2067 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2067"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , const D : usize > From < [Translation < T :: Element , D > ; 8] > for Translation < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 8] > , T :: Element : Scalar , { # [inline] fn from (arr : [Translation < T :: Element , D > ; 8]) -> Self { Self :: from (OVector :: from ([arr [0] . vector . clone () , arr [1] . vector . clone () , arr [2] . vector . clone () , arr [3] . vector . clone () , arr [4] . vector . clone () , arr [5] . vector . clone () , arr [6] . vector . clone () , arr [7] . vector . clone () ,])) } }
};
}
