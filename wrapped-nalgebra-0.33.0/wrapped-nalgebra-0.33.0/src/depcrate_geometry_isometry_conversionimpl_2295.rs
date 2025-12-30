// Generated macro for impl_2295 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2295 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2295"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > From < SVector < T , D > > for Isometry < T , R , D > where R : AbstractRotation < T , D > , { # [inline] fn from (coords : SVector < T , D >) -> Self { Self :: from_parts (coords . into () , R :: identity ()) } }
};
}
