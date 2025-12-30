// Generated macro for impl_2065 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2065 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2065"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , const D : usize > From < [Translation < T :: Element , D > ; 2] > for Translation < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 2] > , T :: Element : Scalar , { # [inline] fn from (arr : [Translation < T :: Element , D > ; 2]) -> Self { Self :: from (OVector :: from ([arr [0] . vector . clone () , arr [1] . vector . clone () ,])) } }
};
}
