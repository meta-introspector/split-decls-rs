// Generated macro for impl_1943 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1943 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1943"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue > From < [UnitComplex < T :: Element > ; 4] > for UnitComplex < T > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 4] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [UnitComplex < T :: Element > ; 4]) -> Self { Self :: new_unchecked (Complex { re : T :: from ([arr [0] . re , arr [1] . re , arr [2] . re , arr [3] . re]) , im : T :: from ([arr [0] . im , arr [1] . im , arr [2] . im , arr [3] . im]) , }) } }
};
}
