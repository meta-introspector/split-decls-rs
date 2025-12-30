// Generated macro for impl_43 (impl)
macro_rules! Depcrate_checkedimpl_43 {
() => {
// Module: crate::checked
// Provides: {"impl_43"}
// Dependencies: {}
impl < T > Checked < T > { # [doc = " Create a new checked arithmetic wrapper for the given value."] pub fn new (val : T) -> Self { Self (CtOption :: new (val , Choice :: from (1))) } }
};
}
