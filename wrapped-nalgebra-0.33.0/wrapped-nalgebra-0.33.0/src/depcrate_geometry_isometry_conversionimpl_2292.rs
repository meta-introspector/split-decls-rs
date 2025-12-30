// Generated macro for impl_2292 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2292 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2292"}
// Dependencies: {}
impl < T : SimdRealField , R : AbstractRotation < T , D > , const D : usize > From < Translation < T , D > > for Isometry < T , R , D > { # [inline] fn from (tra : Translation < T , D >) -> Self { Self :: from_parts (tra , R :: identity ()) } }
};
}
