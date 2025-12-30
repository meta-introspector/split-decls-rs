// Generated macro for impl_18 (impl)
macro_rules! Depcrate_poolimpl_18 {
() => {
// Module: crate::pool
// Provides: {"impl_18"}
// Dependencies: {}
impl PoolState { fn push (& self , worker : Worker) { worker . set_onmessage (Some (self . callback . as_ref () . unchecked_ref ())) ; worker . set_onerror (Some (self . callback . as_ref () . unchecked_ref ())) ; let mut workers = self . workers . borrow_mut () ; for prev in workers . iter () { let prev : & JsValue = prev ; let worker : & JsValue = & worker ; assert ! (prev != worker) ; } workers . push (worker) ; } }
};
}
