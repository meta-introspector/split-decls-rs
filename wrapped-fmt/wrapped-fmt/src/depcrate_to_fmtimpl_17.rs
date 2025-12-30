// Generated macro for impl_17 (impl)
macro_rules! Depcrate_to_fmtimpl_17 {
() => {
// Module: crate::to_fmt
// Provides: {"impl_17"}
// Dependencies: {}
impl < V : sval :: Value + ? Sized > ToFmt < V > { # [doc = "\n    Adapt a reference to an [`sval::Value`] into a [`fmt::Debug`] or [`fmt::Display`].\n    "] pub fn new_borrowed < 'a > (value : & 'a V) -> & 'a ToFmt < V > { unsafe { & * (value as * const _ as * const ToFmt < V >) } } }
};
}
