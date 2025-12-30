// Generated macro for impl_928 (impl)
macro_rules! Depcrate_base_matriximpl_928 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_928"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S > RelativeEq for Matrix < T , R , C , S > where T : Scalar + RelativeEq , S : Storage < T , R , C > , T :: Epsilon : Clone , { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . relative_eq (other , epsilon , max_relative) } }
};
}
