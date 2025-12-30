// Generated macro for impl_3203 (impl)
macro_rules! Depcrate_third_party_alga_alga_pointimpl_3203 {
() => {
// Module: crate::third_party::alga::alga_point
// Provides: {"impl_3203"}
// Dependencies: {}
impl < T , const D : usize > MeetSemilattice for Point < T , D > where T : Scalar + MeetSemilattice , { # [inline] fn meet (& self , other : & Self) -> Self { Self :: from (self . coords . meet (& other . coords)) } }
};
}
