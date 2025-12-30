// Generated macro for impl_2066 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2066 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2066"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , const D : usize > From < [Translation < T :: Element , D > ; 4] > for Translation < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 4] > , T :: Element : Scalar , { # [inline] fn from (arr : [Translation < T :: Element , D > ; 4]) -> Self { Self :: from (OVector :: from ([arr [0] . vector . clone () , arr [1] . vector . clone () , arr [2] . vector . clone () , arr [3] . vector . clone () ,])) } }
};
}
