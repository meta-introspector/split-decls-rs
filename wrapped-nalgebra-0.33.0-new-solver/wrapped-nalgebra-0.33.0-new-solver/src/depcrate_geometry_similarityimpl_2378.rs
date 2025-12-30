// Generated macro for impl_2378 (impl)
macro_rules! Depcrate_geometry_similarityimpl_2378 {
() => {
// Module: crate::geometry::similarity
// Provides: {"impl_2378"}
// Dependencies: {}
impl < T : RealField , R , const D : usize > RelativeEq for Similarity < T , R , D > where R : AbstractRotation < T , D > + RelativeEq < Epsilon = T :: Epsilon > , T :: Epsilon : Clone , { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . isometry . relative_eq (& other . isometry , epsilon . clone () , max_relative . clone ()) && self . scaling . relative_eq (& other . scaling , epsilon , max_relative) } }
};
}
