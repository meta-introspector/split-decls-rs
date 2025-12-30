// Generated macro for impl_2181 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2181 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2181"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , const D : usize > From < [Scale < T :: Element , D > ; 8] > for Scale < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 8] > , T :: Element : Scalar , { # [inline] fn from (arr : [Scale < T :: Element , D > ; 8]) -> Self { Self :: from (OVector :: from ([arr [0] . vector . clone () , arr [1] . vector . clone () , arr [2] . vector . clone () , arr [3] . vector . clone () , arr [4] . vector . clone () , arr [5] . vector . clone () , arr [6] . vector . clone () , arr [7] . vector . clone () ,])) } }
};
}
