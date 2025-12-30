// Generated macro for impl_29 (impl)
macro_rules! Depcrate_to_valueimpl_29 {
() => {
// Module: crate::to_value
// Provides: {"impl_29"}
// Dependencies: {}
impl < V : fmt :: Display > DisplayToValue < V > { # [doc = "\n    Adapt a [`fmt::Display`] into an [`sval::Value`].\n    "] pub const fn new (value : V) -> DisplayToValue < V > { DisplayToValue (value) } }
};
}
