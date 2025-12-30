// Generated macro for impl_27 (impl)
macro_rules! Depcrate_to_valueimpl_27 {
() => {
// Module: crate::to_value
// Provides: {"impl_27"}
// Dependencies: {}
impl < V : fmt :: Debug + ? Sized > DebugToValue < V > { # [doc = "\n    Adapt a reference to a [`fmt::Debug`] into an [`sval::Value`].\n    "] pub const fn new_borrowed < 'a > (value : & 'a V) -> & 'a DebugToValue < V > { unsafe { & * (value as * const _ as * const DebugToValue < V >) } } }
};
}
