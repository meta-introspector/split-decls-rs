// Generated macro for impl_2180 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2180 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2180"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , const D : usize > From < [Scale < T :: Element , D > ; 4] > for Scale < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 4] > , T :: Element : Scalar , { # [inline] fn from (arr : [Scale < T :: Element , D > ; 4]) -> Self { Self :: from (OVector :: from ([arr [0] . vector . clone () , arr [1] . vector . clone () , arr [2] . vector . clone () , arr [3] . vector . clone () ,])) } }
};
}
