// Generated macro for impl_321 (impl)
macro_rules! Depcrate_errorimpl_321 {
() => {
// Module: crate::error
// Provides: {"impl_321"}
// Dependencies: {}
impl Iterator for IntoIterEnum { type Item = Error ; fn next (& mut self) -> Option < Self :: Item > { match * self { IntoIterEnum :: Single (ref mut content) => content . next () , IntoIterEnum :: Multiple (ref mut content) => content . next () , } } }
};
}
