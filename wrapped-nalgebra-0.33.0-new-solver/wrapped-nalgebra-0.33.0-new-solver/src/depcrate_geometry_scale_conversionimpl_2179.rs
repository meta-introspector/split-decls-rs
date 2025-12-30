// Generated macro for impl_2179 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2179 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2179"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , const D : usize > From < [Scale < T :: Element , D > ; 2] > for Scale < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 2] > , T :: Element : Scalar , { # [inline] fn from (arr : [Scale < T :: Element , D > ; 2]) -> Self { Self :: from (OVector :: from ([arr [0] . vector . clone () , arr [1] . vector . clone () ,])) } }
};
}
