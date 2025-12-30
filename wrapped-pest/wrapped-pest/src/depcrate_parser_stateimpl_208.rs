// Generated macro for impl_208 (impl)
macro_rules! Depcrate_parser_stateimpl_208 {
() => {
// Module: crate::parser_state
// Provides: {"impl_208"}
// Dependencies: {}
impl < 'i > SpanOrLiteral < 'i > { # [inline] fn as_borrowed_or_rc (& self) -> BorrowedOrArc < 'i > { match self { Self :: Span (s) => BorrowedOrArc :: Borrowed (s . as_str ()) , Self :: Literal (s) => s . clone () , } } }
};
}
