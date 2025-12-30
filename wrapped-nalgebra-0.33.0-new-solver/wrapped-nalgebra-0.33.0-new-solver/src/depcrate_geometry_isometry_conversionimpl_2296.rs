// Generated macro for impl_2296 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2296 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2296"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > From < Point < T , D > > for Isometry < T , R , D > where R : AbstractRotation < T , D > , { # [inline] fn from (coords : Point < T , D >) -> Self { Self :: from_parts (coords . into () , R :: identity ()) } }
};
}
