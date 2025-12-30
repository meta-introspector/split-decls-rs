// Generated macro for impl_144 (impl)
macro_rules! Depcrate_astimpl_144 {
() => {
// Module: crate::ast
// Provides: {"impl_144"}
// Dependencies: {}
impl < N : AsRef < str > > Type < N > { # [doc = " Strips this [`Type`] from [`NonNull`], returning it as a `null`able one."] pub (crate) fn into_nullable (mut self) -> Self { if self . is_non_null () { self . modifiers . pop () ; } self } }
};
}
