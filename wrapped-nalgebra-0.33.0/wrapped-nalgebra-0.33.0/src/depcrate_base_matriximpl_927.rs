// Generated macro for impl_927 (impl)
macro_rules! Depcrate_base_matriximpl_927 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_927"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S > AbsDiffEq for Matrix < T , R , C , S > where T : Scalar + AbsDiffEq , S : RawStorage < T , R , C > , T :: Epsilon : Clone , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . iter () . zip (other . iter ()) . all (| (a , b) | a . abs_diff_eq (b , epsilon . clone ())) } }
};
}
