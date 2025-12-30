// Generated macro for impl_1945 (impl)
macro_rules! Depcrate_geometry_unit_complex_conversionimpl_1945 {
() => {
// Module: crate::geometry::unit_complex_conversion
// Provides: {"impl_1945"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue > From < [UnitComplex < T :: Element > ; 16] > for UnitComplex < T > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 16] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [UnitComplex < T :: Element > ; 16]) -> Self { Self :: new_unchecked (Complex { re : T :: from ([arr [0] . re , arr [1] . re , arr [2] . re , arr [3] . re , arr [4] . re , arr [5] . re , arr [6] . re , arr [7] . re , arr [8] . re , arr [9] . re , arr [10] . re , arr [11] . re , arr [12] . re , arr [13] . re , arr [14] . re , arr [15] . re ,]) , im : T :: from ([arr [0] . im , arr [1] . im , arr [2] . im , arr [3] . im , arr [4] . im , arr [5] . im , arr [6] . im , arr [7] . im , arr [8] . im , arr [9] . im , arr [10] . im , arr [11] . im , arr [12] . im , arr [13] . im , arr [14] . im , arr [15] . im ,]) , }) } }
};
}
