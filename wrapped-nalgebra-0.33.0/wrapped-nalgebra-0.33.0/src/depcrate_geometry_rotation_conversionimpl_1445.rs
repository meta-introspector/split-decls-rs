// Generated macro for impl_1445 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1445 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1445"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , const D : usize > From < [Rotation < T :: Element , D > ; 8] > for Rotation < T , D > where T : From < [< T as SimdValue > :: Element ; 8] > , T :: Element : Scalar + Copy , { # [inline] fn from (arr : [Rotation < T :: Element , D > ; 8]) -> Self { Self :: from_matrix_unchecked (OMatrix :: from ([arr [0] . into_inner () , arr [1] . into_inner () , arr [2] . into_inner () , arr [3] . into_inner () , arr [4] . into_inner () , arr [5] . into_inner () , arr [6] . into_inner () , arr [7] . into_inner () ,])) } }
};
}
