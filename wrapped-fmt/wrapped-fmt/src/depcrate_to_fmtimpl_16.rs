// Generated macro for impl_16 (impl)
macro_rules! Depcrate_to_fmtimpl_16 {
() => {
// Module: crate::to_fmt
// Provides: {"impl_16"}
// Dependencies: {}
impl < V : sval :: Value > ToFmt < V > { # [doc = "\n    Adapt an [`sval::Value`] into a [`fmt::Debug`] or [`fmt::Display`].\n    "] pub fn new (value : V) -> ToFmt < V > { ToFmt (value) } }
};
}
