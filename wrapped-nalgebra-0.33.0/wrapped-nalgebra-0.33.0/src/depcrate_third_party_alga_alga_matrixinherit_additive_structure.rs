// Generated macro for inherit_additive_structure (macro)
macro_rules! Depcrate_third_party_alga_alga_matrixinherit_additive_structure {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"inherit_additive_structure"}
// Dependencies: {}
macro_rules ! inherit_additive_structure (($ ($ marker : ident <$ operator : ident > $ (+ $ bounds : ident) *) ,* $ (,) *) => { $ (impl < T , R : DimName , C : DimName > $ marker <$ operator > for OMatrix < T , R , C > where T : Scalar + $ marker <$ operator > $ (+ $ bounds) *, DefaultAllocator : Allocator < R , C > { }) * }) ;
};
}
