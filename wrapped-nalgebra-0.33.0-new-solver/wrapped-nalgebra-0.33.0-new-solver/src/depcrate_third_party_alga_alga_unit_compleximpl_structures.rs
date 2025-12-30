// Generated macro for impl_structures (macro)
macro_rules! Depcrate_third_party_alga_alga_unit_compleximpl_structures {
() => {
// Module: crate::third_party::alga::alga_unit_complex
// Provides: {"impl_structures"}
// Dependencies: {}
macro_rules ! impl_structures (($ ($ marker : ident <$ operator : ident >) ,* $ (,) *) => { $ (impl < T : RealField + simba :: scalar :: RealField > $ marker <$ operator > for UnitComplex < T > { }) * }) ;
};
}
