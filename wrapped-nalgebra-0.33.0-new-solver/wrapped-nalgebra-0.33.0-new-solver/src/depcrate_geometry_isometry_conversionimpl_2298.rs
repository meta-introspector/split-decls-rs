// Generated macro for impl_2298 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2298 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2298"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , R , const D : usize > From < [Isometry < T :: Element , R :: Element , D > ; 4] > for Isometry < T , R , D > where T : From < [< T as SimdValue > :: Element ; 4] > , R : SimdValue + AbstractRotation < T , D > + From < [< R as SimdValue > :: Element ; 4] > , R :: Element : AbstractRotation < T :: Element , D > , T :: Element : Scalar + Copy , R :: Element : Scalar + Copy , { # [inline] fn from (arr : [Isometry < T :: Element , R :: Element , D > ; 4]) -> Self { let tra = Translation :: from ([arr [0] . translation , arr [1] . translation , arr [2] . translation , arr [3] . translation ,]) ; let rot = R :: from ([arr [0] . rotation , arr [1] . rotation , arr [2] . rotation , arr [3] . rotation ,]) ; Self :: from_parts (tra , rot) } }
};
}
