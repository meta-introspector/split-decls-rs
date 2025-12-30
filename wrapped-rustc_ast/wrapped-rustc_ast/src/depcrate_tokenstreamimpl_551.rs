// Generated macro for impl_551 (impl)
macro_rules! Depcrate_tokenstreamimpl_551 {
() => {
// Module: crate::tokenstream
// Provides: {"impl_551"}
// Dependencies: {}
impl DelimSpan { pub fn from_single (sp : Span) -> Self { DelimSpan { open : sp , close : sp } } pub fn from_pair (open : Span , close : Span) -> Self { DelimSpan { open , close } } pub fn dummy () -> Self { Self :: from_single (DUMMY_SP) } pub fn entire (self) -> Span { self . open . with_hi (self . close . hi ()) } }
};
}
