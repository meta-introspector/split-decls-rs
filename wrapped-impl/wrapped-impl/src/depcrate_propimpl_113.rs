// Generated macro for impl_113 (impl)
macro_rules! Depcrate_propimpl_113 {
() => {
// Module: crate::prop
// Provides: {"impl_113"}
// Dependencies: {}
impl Field < '_ > { pub (crate) fn is_backtrace (& self) -> bool { type_is_backtrace (self . ty) } pub (crate) fn source_span (& self) -> Span { if let Some (source_attr) = & self . attrs . source { source_attr . span } else if let Some (from_attr) = & self . attrs . from { from_attr . span } else { self . member . span () } } }
};
}
