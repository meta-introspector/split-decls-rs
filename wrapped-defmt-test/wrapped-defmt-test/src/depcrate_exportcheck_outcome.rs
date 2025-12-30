// Generated macro for check_outcome (function)
macro_rules! Depcrate_exportcheck_outcome {
() => {
// Module: crate::export
// Provides: {"check_outcome"}
// Dependencies: {}
pub fn check_outcome < T : TestOutcome > (outcome : T , should_error : bool) { if outcome . is_success () == should_error { let note = if should_error { defmt :: intern ! ("`#[should_error]` ") } else { defmt :: intern ! ("") } ; defmt :: panic ! ("{}test failed with outcome: {}" , note , outcome) ; } }
};
}
