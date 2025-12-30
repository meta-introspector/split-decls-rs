// Generated macro for impl_39 (impl)
macro_rules! Depcrate_relative_eqimpl_39 {
() => {
// Module: crate::relative_eq
// Provides: {"impl_39"}
// Dependencies: {}
impl < A , B > RelativeEq < [B] > for [A] where A : RelativeEq < B > , A :: Epsilon : Clone , { # [inline] fn default_max_relative () -> A :: Epsilon { A :: default_max_relative () } # [inline] fn relative_eq (& self , other : & [B] , epsilon : A :: Epsilon , max_relative : A :: Epsilon) -> bool { self . len () == other . len () && Iterator :: zip (self . iter () , other) . all (| (x , y) | A :: relative_eq (x , y , epsilon . clone () , max_relative . clone ())) } }
};
}
