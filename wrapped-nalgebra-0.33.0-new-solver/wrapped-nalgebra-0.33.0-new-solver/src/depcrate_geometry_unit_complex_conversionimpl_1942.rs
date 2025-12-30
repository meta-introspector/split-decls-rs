// Generated macro for impl_1942 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1942 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1942"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue > From < [UnitComplex < T :: Element > ; 2] > for UnitComplex < T > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 2] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [UnitComplex < T :: Element > ; 2]) -> Self { Self :: new_unchecked (Complex { re : T :: from ([arr [0] . re , arr [1] . re]) , im : T :: from ([arr [0] . im , arr [1] . im]) , }) } }
};
}
