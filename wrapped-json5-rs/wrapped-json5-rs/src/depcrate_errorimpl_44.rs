// Generated macro for impl_44 (impl)
macro_rules! Depcrate_errorimpl_44 {
() => {
// Module: crate::error
// Provides: {"impl_44"}
// Dependencies: {}
impl From < & Span < '_ > > for Location { fn from (s : & Span < '_ >) -> Self { let (line , column) = s . start_pos () . line_col () ; Self { line , column } } }
};
}
