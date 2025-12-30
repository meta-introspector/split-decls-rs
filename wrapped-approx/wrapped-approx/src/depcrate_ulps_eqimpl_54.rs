// Generated macro for impl_54 (impl)
macro_rules! Depcrate_ulps_eqimpl_54 {
() => {
// Module: crate::ulps_eq
// Provides: {"impl_54"}
// Dependencies: {}
impl < A , B > UlpsEq < [B] > for [A] where A : UlpsEq < B > , A :: Epsilon : Clone , { # [inline] fn default_max_ulps () -> u32 { A :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & [B] , epsilon : A :: Epsilon , max_ulps : u32) -> bool { self . len () == other . len () && Iterator :: zip (self . iter () , other) . all (| (x , y) | A :: ulps_eq (x , y , epsilon . clone () , max_ulps)) } }
};
}
