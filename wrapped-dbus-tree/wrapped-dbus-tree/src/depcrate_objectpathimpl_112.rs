// Generated macro for impl_112 (impl)
macro_rules! Depcrate_objectpathimpl_112 {
() => {
// Module: crate::objectpath
// Provides: {"impl_112"}
// Dependencies: {}
impl < 'a , I : Iterator < Item = ConnectionItem > , M : 'a + MethodType < D > , D : DataType + 'a > Iterator for TreeServer < 'a , I , M , D > { type Item = ConnectionItem ; fn next (& mut self) -> Option < ConnectionItem > { loop { let n = self . iter . next () ; if let Some (ConnectionItem :: MethodCall (ref msg)) = n { if let Some (v) = self . tree . handle (& msg) { for m in v { let _ = self . conn . send (m) ; } ; continue ; } } return n ; } } }
};
}
