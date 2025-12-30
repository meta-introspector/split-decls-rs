// Generated macro for impl_529 (impl)
macro_rules! Depcrate_setimpl_529 {
() => {
// Module: crate::set
// Provides: {"impl_529"}
// Dependencies: {}
impl < T , S , A : Allocator > Clone for Intersection < '_ , T , S , A > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Intersection { iter : self . iter . clone () , .. * self } } }
};
}
