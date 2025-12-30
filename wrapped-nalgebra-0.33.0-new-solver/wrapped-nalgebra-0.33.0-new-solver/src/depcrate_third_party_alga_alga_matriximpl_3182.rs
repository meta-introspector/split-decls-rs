// Generated macro for impl_3182 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3182 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3182"}
// Dependencies: {}
impl < T , R : DimName , C : DimName > AbstractModule for OMatrix < T , R , C > where T : Scalar + RingCommutative , DefaultAllocator : Allocator < R , C > , { type AbstractRing = T ; # [inline] fn multiply_by (& self , n : T) -> Self { self * n } }
};
}
