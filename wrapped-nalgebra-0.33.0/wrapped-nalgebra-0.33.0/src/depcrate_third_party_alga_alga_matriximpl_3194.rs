// Generated macro for impl_3194 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3194 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3194"}
// Dependencies: {}
impl < T , R : Dim , C : Dim > JoinSemilattice for OMatrix < T , R , C > where T : Scalar + JoinSemilattice , DefaultAllocator : Allocator < R , C > , { # [inline] fn join (& self , other : & Self) -> Self { self . zip_map (other , | a , b | a . join (& b)) } }
};
}
