// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl Span { # [doc = " Execute `f` in with this span active, consuming it."] pub fn into_scope < T > (self , f : impl FnOnce () -> T) -> T { f () } }
};
}
