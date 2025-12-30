// Generated macro for impl_3190 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3190 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3190"}
// Dependencies: {}
impl < T , D : DimName > AbstractMagma < Multiplicative > for OMatrix < T , D , D > where T : Scalar + Zero + One + ClosedAdd + ClosedMul , DefaultAllocator : Allocator < D , D > , { # [inline] fn operate (& self , other : & Self) -> Self { self * other } }
};
}
