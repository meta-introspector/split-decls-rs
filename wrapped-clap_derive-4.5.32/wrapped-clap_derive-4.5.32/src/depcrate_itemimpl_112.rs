// Generated macro for impl_112 (impl)
macro_rules! Depcrate_itemimpl_112 {
() => {
// Module: crate::item
// Provides: {"impl_112"}
// Dependencies: {}
impl ValueParser { fn resolve (self , _inner_type : & Type) -> Method { match self { Self :: Explicit (method) => method , Self :: Implicit (ident) => default_value_parser (_inner_type , ident . span ()) , } } fn span (& self) -> Span { match self { Self :: Explicit (method) => method . name . span () , Self :: Implicit (ident) => ident . span () , } } }
};
}
