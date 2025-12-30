// Generated macro for IntoResult (trait)
macro_rules! Depcrate_export_traitsIntoResult {
() => {
// Module: crate::export::traits
// Provides: {"IntoResult"}
// Dependencies: {}
# [doc = " Transform `self` into a `Result`"] # [doc = ""] # [doc = " # Call sites"] # [doc = " * [`defmt::unwrap!`]"] pub trait IntoResult { type Ok ; type Error ; fn into_result (self) -> Result < Self :: Ok , Self :: Error > ; }
};
}
