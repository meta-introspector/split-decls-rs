// Generated macro for impl_3205 (impl)
macro_rules! Depcrate_third_party_alga_alga_pointimpl_3205 {
() => {
// Module: crate::third_party::alga::alga_point
// Provides: {"impl_3205"}
// Dependencies: {}
impl < T , const D : usize > Lattice for Point < T , D > where T : Scalar + Lattice , { # [inline] fn meet_join (& self , other : & Self) -> (Self , Self) { let (meet , join) = self . coords . meet_join (& other . coords) ; (Self :: from (meet) , Self :: from (join)) } }
};
}
