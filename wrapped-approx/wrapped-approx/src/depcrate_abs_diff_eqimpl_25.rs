// Generated macro for impl_25 (impl)
macro_rules! Depcrate_abs_diff_eqimpl_25 {
() => {
// Module: crate::abs_diff_eq
// Provides: {"impl_25"}
// Dependencies: {}
impl < A , B > AbsDiffEq < [B] > for [A] where A : AbsDiffEq < B > , A :: Epsilon : Clone , { type Epsilon = A :: Epsilon ; # [inline] fn default_epsilon () -> A :: Epsilon { A :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & [B] , epsilon : A :: Epsilon) -> bool { self . len () == other . len () && Iterator :: zip (self . iter () , other) . all (| (x , y) | A :: abs_diff_eq (x , y , epsilon . clone ())) } }
};
}
