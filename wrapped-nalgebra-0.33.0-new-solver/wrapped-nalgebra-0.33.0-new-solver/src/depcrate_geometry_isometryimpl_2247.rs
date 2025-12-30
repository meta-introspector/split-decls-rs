// Generated macro for impl_2247 (impl)
macro_rules! Depcrate_geometry_isometryimpl_2247 {
() => {
// Module: crate::geometry::isometry
// Provides: {"impl_2247"}
// Dependencies: {}
impl < T : RealField , R , const D : usize > RelativeEq for Isometry < T , R , D > where R : AbstractRotation < T , D > + RelativeEq < Epsilon = T :: Epsilon > , T :: Epsilon : Clone , { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . translation . relative_eq (& other . translation , epsilon . clone () , max_relative . clone ()) && self . rotation . relative_eq (& other . rotation , epsilon , max_relative) } }
};
}
