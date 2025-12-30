// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl Future for JsFuture { type Output = Result < JsValue , JsValue > ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { let mut inner = self . inner . borrow_mut () ; if let Some (val) = inner . result . take () { return Poll :: Ready (val) ; } inner . task = Some (cx . waker () . clone ()) ; Poll :: Pending } }
};
}
