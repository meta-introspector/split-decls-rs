// Generated macro for impl_2245 (impl)
macro_rules! Depcrate_geometry_isometryimpl_2245 {
() => {
// Module: crate::geometry::isometry
// Provides: {"impl_2245"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > PartialEq for Isometry < T , R , D > where R : AbstractRotation < T , D > + PartialEq , { # [inline] fn eq (& self , right : & Self) -> bool { self . translation == right . translation && self . rotation == right . rotation } }
};
}
