// Generated macro for impl_947 (impl)
macro_rules! Depcrate_base_matriximpl_947 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_947"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S > RelativeEq for Unit < Matrix < T , R , C , S > > where T : Scalar + RelativeEq , S : Storage < T , R , C > , T :: Epsilon : Clone , { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . as_ref () . relative_eq (other . as_ref () , epsilon , max_relative) } }
};
}
