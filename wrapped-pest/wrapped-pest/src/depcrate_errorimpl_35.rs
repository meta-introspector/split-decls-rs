// Generated macro for impl_35 (impl)
macro_rules! Depcrate_errorimpl_35 {
() => {
// Module: crate::error
// Provides: {"impl_35"}
// Dependencies: {}
impl From < Span < '_ > > for LineColLocation { fn from (value : Span < '_ >) -> Self { let (start , end) = value . split () ; Self :: Span (start . line_col () , end . line_col ()) } }
};
}
