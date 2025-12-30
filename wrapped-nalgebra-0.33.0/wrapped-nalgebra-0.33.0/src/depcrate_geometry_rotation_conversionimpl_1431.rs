// Generated macro for impl_1431 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1431 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1431"}
// Dependencies: {}
impl < T1 , T2 , const D : usize > SubsetOf < Rotation < T2 , D > > for Rotation < T1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> Rotation < T2 , D > { Rotation :: from_matrix_unchecked (self . matrix () . to_superset ()) } # [inline] fn is_in_subset (rot : & Rotation < T2 , D >) -> bool { crate :: is_convertible :: < _ , SMatrix < T1 , D , D > > (rot . matrix ()) } # [inline] fn from_superset_unchecked (rot : & Rotation < T2 , D >) -> Self { Rotation :: from_matrix_unchecked (rot . matrix () . to_subset_unchecked ()) } }
};
}
