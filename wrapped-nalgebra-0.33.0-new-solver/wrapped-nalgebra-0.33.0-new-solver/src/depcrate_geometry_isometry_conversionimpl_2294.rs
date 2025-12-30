// Generated macro for impl_2294 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2294 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2294"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > From < [T ; D] > for Isometry < T , R , D > where R : AbstractRotation < T , D > , { # [inline] fn from (coords : [T ; D]) -> Self { Self :: from_parts (coords . into () , R :: identity ()) } }
};
}
