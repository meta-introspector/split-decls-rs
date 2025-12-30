// Generated macro for impl_653 (impl)
macro_rules! Depcrate_traitsimpl_653 {
() => {
// Module: crate::traits
// Provides: {"impl_653"}
// Dependencies: {}
impl < 'tcx > PolyTraitObligation < 'tcx > { pub fn derived_cause (& self , variant : impl FnOnce (DerivedCause < 'tcx >) -> ObligationCauseCode < 'tcx > ,) -> ObligationCause < 'tcx > { self . cause . clone () . derived_cause (self . predicate , variant) } }
};
}
