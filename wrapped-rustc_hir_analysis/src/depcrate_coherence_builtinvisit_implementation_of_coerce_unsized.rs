// Generated macro for visit_implementation_of_coerce_unsized (function)
macro_rules! Depcrate_coherence_builtinvisit_implementation_of_coerce_unsized {
() => {
// Module: crate::coherence::builtin
// Provides: {"visit_implementation_of_coerce_unsized"}
// Dependencies: {}
fn visit_implementation_of_coerce_unsized (checker : & Checker < '_ >) -> Result < () , ErrorGuaranteed > { let tcx = checker . tcx ; let impl_did = checker . impl_def_id ; debug ! ("visit_implementation_of_coerce_unsized: impl_did={:?}" , impl_did) ; tcx . ensure_ok () . coerce_unsized_info (impl_did) }
};
}
