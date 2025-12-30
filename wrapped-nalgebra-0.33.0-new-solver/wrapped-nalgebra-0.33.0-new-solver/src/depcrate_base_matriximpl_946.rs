// Generated macro for impl_946 (impl)
macro_rules! Depcrate_base_matriximpl_946 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_946"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S > AbsDiffEq for Unit < Matrix < T , R , C , S > > where T : Scalar + AbsDiffEq , S : RawStorage < T , R , C > , T :: Epsilon : Clone , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . as_ref () . abs_diff_eq (other . as_ref () , epsilon) } }
};
}
