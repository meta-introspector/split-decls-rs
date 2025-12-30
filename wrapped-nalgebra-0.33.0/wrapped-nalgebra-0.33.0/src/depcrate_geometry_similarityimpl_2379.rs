// Generated macro for impl_2379 (impl)
macro_rules! Depcrate_geometry_similarityimpl_2379 {
() => {
// Module: crate::geometry::similarity
// Provides: {"impl_2379"}
// Dependencies: {}
impl < T : RealField , R , const D : usize > UlpsEq for Similarity < T , R , D > where R : AbstractRotation < T , D > + UlpsEq < Epsilon = T :: Epsilon > , T :: Epsilon : Clone , { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . isometry . ulps_eq (& other . isometry , epsilon . clone () , max_ulps) && self . scaling . ulps_eq (& other . scaling , epsilon , max_ulps) } }
};
}
