// Generated macro for impl_2287 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2287 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2287"}
// Dependencies: {}
impl < T1 , T2 , R1 , R2 , const D : usize > SubsetOf < Isometry < T2 , R2 , D > > for Isometry < T1 , R1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , R1 : AbstractRotation < T1 , D > + SubsetOf < R2 > , R2 : AbstractRotation < T2 , D > , { # [inline] fn to_superset (& self) -> Isometry < T2 , R2 , D > { Isometry :: from_parts (self . translation . to_superset () , self . rotation . to_superset ()) } # [inline] fn is_in_subset (iso : & Isometry < T2 , R2 , D >) -> bool { crate :: is_convertible :: < _ , Translation < T1 , D > > (& iso . translation) && crate :: is_convertible :: < _ , R1 > (& iso . rotation) } # [inline] fn from_superset_unchecked (iso : & Isometry < T2 , R2 , D >) -> Self { Isometry :: from_parts (iso . translation . to_subset_unchecked () , iso . rotation . to_subset_unchecked () ,) } }
};
}
