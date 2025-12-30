// Generated macro for impl_372 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_372 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_372"}
// Dependencies: {}
impl < T : Poolable , K : Key > Future for Checkout < T , K > { type Output = Result < Pooled < T , K > , Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { if let Some (pooled) = ready ! (self . poll_waiter (cx) ?) { return Poll :: Ready (Ok (pooled)) ; } if let Some (pooled) = self . checkout (cx) { Poll :: Ready (Ok (pooled)) } else if ! self . pool . is_enabled () { Poll :: Ready (Err (Error :: PoolDisabled)) } else { debug_assert ! (self . waiter . is_some ()) ; Poll :: Pending } } }
};
}
