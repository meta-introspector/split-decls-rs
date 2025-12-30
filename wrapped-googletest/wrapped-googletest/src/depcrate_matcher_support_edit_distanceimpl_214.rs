// Generated macro for impl_214 (impl)
macro_rules! Depcrate_matcher_support_edit_distanceimpl_214 {
() => {
// Module: crate::matcher_support::edit_distance
// Provides: {"impl_214"}
// Dependencies: {}
impl < T : Clone > Path < T > { fn extend_actual_endpoint (& self) -> Self { Self { actual_endpoint : self . actual_endpoint + 1 , expected_endpoint : self . expected_endpoint , edits : self . edits . clone () , } } }
};
}
