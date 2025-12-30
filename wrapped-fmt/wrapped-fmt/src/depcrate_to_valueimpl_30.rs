// Generated macro for impl_30 (impl)
macro_rules! Depcrate_to_valueimpl_30 {
() => {
// Module: crate::to_value
// Provides: {"impl_30"}
// Dependencies: {}
impl < V : fmt :: Display + ? Sized > DisplayToValue < V > { # [doc = "\n    Adapt a reference to a [`fmt::Display`] into an [`sval::Value`].\n    "] pub const fn new_borrowed < 'a > (value : & 'a V) -> & 'a DisplayToValue < V > { unsafe { & * (value as * const _ as * const DisplayToValue < V >) } } }
};
}
