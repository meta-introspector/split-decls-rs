// Generated macro for impl_130 (impl)
macro_rules! Depcrate_common_span_containerimpl_130 {
() => {
// Module: crate::common::span_container
// Provides: {"impl_130"}
// Dependencies: {}
impl < T > SpanContainer < T > { pub (crate) fn new (ident : Span , expr : Option < Span > , val : T) -> Self { Self { expr , ident , val } } pub (crate) fn span_ident (& self) -> Span { self . ident } pub (crate) fn span_joined (& self) -> Span { if let Some (s) = self . expr { s } else { self . ident } } pub (crate) fn into_inner (self) -> T { self . val } }
};
}
