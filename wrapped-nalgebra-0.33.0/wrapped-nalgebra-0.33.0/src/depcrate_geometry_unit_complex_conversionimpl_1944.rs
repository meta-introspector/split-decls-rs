// Generated macro for impl_1944 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1944 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1944"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue > From < [UnitComplex < T :: Element > ; 8] > for UnitComplex < T > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 8] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [UnitComplex < T :: Element > ; 8]) -> Self { Self :: new_unchecked (Complex { re : T :: from ([arr [0] . re , arr [1] . re , arr [2] . re , arr [3] . re , arr [4] . re , arr [5] . re , arr [6] . re , arr [7] . re ,]) , im : T :: from ([arr [0] . im , arr [1] . im , arr [2] . im , arr [3] . im , arr [4] . im , arr [5] . im , arr [6] . im , arr [7] . im ,]) , }) } }
};
}
