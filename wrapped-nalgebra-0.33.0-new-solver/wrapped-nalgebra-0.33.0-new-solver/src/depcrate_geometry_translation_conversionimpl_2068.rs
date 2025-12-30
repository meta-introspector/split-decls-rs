// Generated macro for impl_2068 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2068 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2068"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , const D : usize > From < [Translation < T :: Element , D > ; 16] > for Translation < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 16] > , T :: Element : Scalar , { # [inline] fn from (arr : [Translation < T :: Element , D > ; 16]) -> Self { Self :: from (OVector :: from ([arr [0] . vector . clone () , arr [1] . vector . clone () , arr [2] . vector . clone () , arr [3] . vector . clone () , arr [4] . vector . clone () , arr [5] . vector . clone () , arr [6] . vector . clone () , arr [7] . vector . clone () , arr [8] . vector . clone () , arr [9] . vector . clone () , arr [10] . vector . clone () , arr [11] . vector . clone () , arr [12] . vector . clone () , arr [13] . vector . clone () , arr [14] . vector . clone () , arr [15] . vector . clone () ,])) } }
};
}
