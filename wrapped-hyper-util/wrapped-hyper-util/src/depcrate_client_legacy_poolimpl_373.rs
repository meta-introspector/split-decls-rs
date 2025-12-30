// Generated macro for impl_373 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_373 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_373"}
// Dependencies: {}
impl < T , K : Key > Drop for Checkout < T , K > { fn drop (& mut self) { if self . waiter . take () . is_some () { trace ! ("checkout dropped for {:?}" , self . key) ; if let Some (Ok (mut inner)) = self . pool . inner . as_ref () . map (| i | i . lock ()) { inner . clean_waiters (& self . key) ; } } } }
};
}
