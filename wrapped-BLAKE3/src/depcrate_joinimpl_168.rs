// Generated macro for impl_168 (impl)
macro_rules! Depcrate_joinimpl_168 {
() => {
// Module: crate::join
// Provides: {"impl_168"}
// Dependencies: {}
# [cfg (feature = "rayon")] impl Join for RayonJoin { # [inline] fn join < A , B , RA , RB > (oper_a : A , oper_b : B) -> (RA , RB) where A : FnOnce () -> RA + Send , B : FnOnce () -> RB + Send , RA : Send , RB : Send , { rayon_core :: join (oper_a , oper_b) } }
};
}
