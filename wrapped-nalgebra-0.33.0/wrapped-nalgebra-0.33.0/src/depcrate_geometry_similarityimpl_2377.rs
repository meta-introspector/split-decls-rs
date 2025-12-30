// Generated macro for impl_2377 (impl)
macro_rules! Depcrate_geometry_similarityimpl_2377 {
() => {
// Module: crate::geometry::similarity
// Provides: {"impl_2377"}
// Dependencies: {}
impl < T : RealField , R , const D : usize > AbsDiffEq for Similarity < T , R , D > where R : AbstractRotation < T , D > + AbsDiffEq < Epsilon = T :: Epsilon > , T :: Epsilon : Clone , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . isometry . abs_diff_eq (& other . isometry , epsilon . clone ()) && self . scaling . abs_diff_eq (& other . scaling , epsilon) } }
};
}
