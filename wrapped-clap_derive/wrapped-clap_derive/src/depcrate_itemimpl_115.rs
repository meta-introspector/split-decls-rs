// Generated macro for impl_115 (impl)
macro_rules! Depcrate_itemimpl_115 {
() => {
// Module: crate::item
// Provides: {"impl_115"}
// Dependencies: {}
impl Action { pub (crate) fn resolve (self , _field_type : & Type) -> Method { match self { Self :: Explicit (method) => method , Self :: Implicit (ident) => default_action (_field_type , ident . span ()) , } } pub (crate) fn span (& self) -> Span { match self { Self :: Explicit (method) => method . name . span () , Self :: Implicit (ident) => ident . span () , } } }
};
}
