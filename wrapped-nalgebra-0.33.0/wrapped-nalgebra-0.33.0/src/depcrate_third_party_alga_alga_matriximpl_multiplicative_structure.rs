// Generated macro for impl_multiplicative_structure (macro)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_multiplicative_structure {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_multiplicative_structure"}
// Dependencies: {}
macro_rules ! impl_multiplicative_structure (($ ($ marker : ident <$ operator : ident > $ (+ $ bounds : ident) *) ,* $ (,) *) => { $ (impl < T , D : DimName > $ marker <$ operator > for OMatrix < T , D , D > where T : Scalar + Zero + One + ClosedAdd + ClosedMul + $ marker <$ operator > $ (+ $ bounds) *, DefaultAllocator : Allocator < D , D > { }) * }) ;
};
}
