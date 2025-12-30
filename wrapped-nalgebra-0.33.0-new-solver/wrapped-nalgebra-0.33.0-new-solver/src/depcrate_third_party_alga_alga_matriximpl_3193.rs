// Generated macro for impl_3193 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3193 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3193"}
// Dependencies: {}
impl < T , R : Dim , C : Dim > MeetSemilattice for OMatrix < T , R , C > where T : Scalar + MeetSemilattice , DefaultAllocator : Allocator < R , C > , { # [inline] fn meet (& self , other : & Self) -> Self { self . zip_map (other , | a , b | a . meet (& b)) } }
};
}
