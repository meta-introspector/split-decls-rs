// Generated macro for impl_1443 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1443 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1443"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , const D : usize > From < [Rotation < T :: Element , D > ; 2] > for Rotation < T , D > where T : From < [< T as SimdValue > :: Element ; 2] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [Rotation < T :: Element , D > ; 2]) -> Self { Self :: from_matrix_unchecked (OMatrix :: from ([arr [0] . into_inner () , arr [1] . into_inner ()])) } }
};
}
