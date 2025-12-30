// Generated macro for impl_2300 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2300 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2300"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , R , const D : usize > From < [Isometry < T :: Element , R :: Element , D > ; 16] > for Isometry < T , R , D > where T : From < [< T as SimdValue > :: Element ; 16] > , R : SimdValue + AbstractRotation < T , D > + From < [< R as SimdValue > :: Element ; 16] > , R :: Element : AbstractRotation < T :: Element , D > , T :: Element : Scalar + Copy , R :: Element : Scalar + Copy , { # [inline] fn from (arr : [Isometry < T :: Element , R :: Element , D > ; 16]) -> Self { let tra = Translation :: from ([arr [0] . translation , arr [1] . translation , arr [2] . translation , arr [3] . translation , arr [4] . translation , arr [5] . translation , arr [6] . translation , arr [7] . translation , arr [8] . translation , arr [9] . translation , arr [10] . translation , arr [11] . translation , arr [12] . translation , arr [13] . translation , arr [14] . translation , arr [15] . translation ,]) ; let rot = R :: from ([arr [0] . rotation , arr [1] . rotation , arr [2] . rotation , arr [3] . rotation , arr [4] . rotation , arr [5] . rotation , arr [6] . rotation , arr [7] . rotation , arr [8] . rotation , arr [9] . rotation , arr [10] . rotation , arr [11] . rotation , arr [12] . rotation , arr [13] . rotation , arr [14] . rotation , arr [15] . rotation ,]) ; Self :: from_parts (tra , rot) } }
};
}
