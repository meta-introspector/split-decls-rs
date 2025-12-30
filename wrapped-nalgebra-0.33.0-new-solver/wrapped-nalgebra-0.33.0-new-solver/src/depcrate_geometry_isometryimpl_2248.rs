// Generated macro for impl_2248 (impl)
macro_rules! Depcrate_geometry_isometryimpl_2248 {
() => {
// Module: crate::geometry::isometry
// Provides: {"impl_2248"}
// Dependencies: {}
impl < T : RealField , R , const D : usize > UlpsEq for Isometry < T , R , D > where R : AbstractRotation < T , D > + UlpsEq < Epsilon = T :: Epsilon > , T :: Epsilon : Clone , { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . translation . ulps_eq (& other . translation , epsilon . clone () , max_ulps) && self . rotation . ulps_eq (& other . rotation , epsilon , max_ulps) } }
};
}
