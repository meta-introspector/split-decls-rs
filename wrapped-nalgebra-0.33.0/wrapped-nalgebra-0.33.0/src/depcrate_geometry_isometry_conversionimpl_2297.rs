// Generated macro for impl_2297 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2297 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2297"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , R , const D : usize > From < [Isometry < T :: Element , R :: Element , D > ; 2] > for Isometry < T , R , D > where T : From < [< T as SimdValue > :: Element ; 2] > , R : SimdValue + AbstractRotation < T , D > + From < [< R as SimdValue > :: Element ; 2] > , R :: Element : AbstractRotation < T :: Element , D > , T :: Element : Scalar + Copy , R :: Element : Scalar + Copy , { # [inline] fn from (arr : [Isometry < T :: Element , R :: Element , D > ; 2]) -> Self { let tra = Translation :: from ([arr [0] . translation , arr [1] . translation]) ; let rot = R :: from ([arr [0] . rotation , arr [0] . rotation]) ; Self :: from_parts (tra , rot) } }
};
}
