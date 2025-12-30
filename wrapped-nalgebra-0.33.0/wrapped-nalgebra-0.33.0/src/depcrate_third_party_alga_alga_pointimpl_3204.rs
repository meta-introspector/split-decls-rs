// Generated macro for impl_3204 (impl)
macro_rules! Depcrate_third_party_alga_alga_pointimpl_3204 {
() => {
// Module: crate::third_party::alga::alga_point
// Provides: {"impl_3204"}
// Dependencies: {}
impl < T , const D : usize > JoinSemilattice for Point < T , D > where T : Scalar + JoinSemilattice , { # [inline] fn join (& self , other : & Self) -> Self { Self :: from (self . coords . join (& other . coords)) } }
};
}
