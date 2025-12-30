// Generated macro for impl_3178 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3178 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3178"}
// Dependencies: {}
impl < T , R : DimName , C : DimName > AbstractMagma < Additive > for OMatrix < T , R , C > where T : Scalar + ClosedAdd , DefaultAllocator : Allocator < R , C > , { # [inline] fn operate (& self , other : & Self) -> Self { self + other } }
};
}
