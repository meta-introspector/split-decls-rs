// Generated macro for marker_impl (macro)
macro_rules! Depcrate_third_party_alga_alga_unit_complexmarker_impl {
() => {
// Module: crate::third_party::alga::alga_unit_complex
// Provides: {"marker_impl"}
// Dependencies: {}
macro_rules ! marker_impl (($ ($ Trait : ident) ,*) => { $ (impl < T : RealField + simba :: scalar :: RealField > $ Trait < Point2 < T >> for UnitComplex < T > { }) * }) ;
};
}
