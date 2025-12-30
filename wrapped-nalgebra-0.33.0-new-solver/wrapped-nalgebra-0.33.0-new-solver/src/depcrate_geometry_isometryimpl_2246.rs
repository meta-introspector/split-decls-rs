// Generated macro for impl_2246 (impl)
macro_rules! Depcrate_geometry_isometryimpl_2246 {
() => {
// Module: crate::geometry::isometry
// Provides: {"impl_2246"}
// Dependencies: {}
impl < T : RealField , R , const D : usize > AbsDiffEq for Isometry < T , R , D > where R : AbstractRotation < T , D > + AbsDiffEq < Epsilon = T :: Epsilon > , T :: Epsilon : Clone , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . translation . abs_diff_eq (& other . translation , epsilon . clone ()) && self . rotation . abs_diff_eq (& other . rotation , epsilon) } }
};
}
