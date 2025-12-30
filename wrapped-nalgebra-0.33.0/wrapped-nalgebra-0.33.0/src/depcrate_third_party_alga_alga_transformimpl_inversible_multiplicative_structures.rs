// Generated macro for impl_inversible_multiplicative_structures (macro)
macro_rules! Depcrate_third_party_alga_alga_transformimpl_inversible_multiplicative_structures {
() => {
// Module: crate::third_party::alga::alga_transform
// Provides: {"impl_inversible_multiplicative_structures"}
// Dependencies: {}
macro_rules ! impl_inversible_multiplicative_structures (($ ($ marker : ident <$ operator : ident >) ,* $ (,) *) => { $ (impl < T : RealField + simba :: scalar :: RealField , C , const D : usize > $ marker <$ operator > for Transform < T , C , D > where Const < D >: DimNameAdd < U1 >, C : SubTCategoryOf < TProjective >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >> { }) * }) ;
};
}
