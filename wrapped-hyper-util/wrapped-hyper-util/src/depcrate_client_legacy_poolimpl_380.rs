// Generated macro for impl_380 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_380 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_380"}
// Dependencies: {}
impl < T : Poolable + 'static , K : Key > IdleTask < T , K > { async fn run (self) { use futures_util :: future ; let mut sleep = self . timer . sleep_until (self . timer . now () + self . duration) ; let mut on_pool_drop = self . pool_drop_notifier ; loop { match future :: select (& mut on_pool_drop , & mut sleep) . await { future :: Either :: Left (_) => { break ; } future :: Either :: Right ((() , _)) => { if let Some (inner) = self . pool . upgrade () { if let Ok (mut inner) = inner . lock () { trace ! ("idle interval checking for expired") ; inner . clear_expired () ; } } let deadline = self . timer . now () + self . duration ; self . timer . reset (& mut sleep , deadline) ; } } } trace ! ("pool closed, canceling idle interval") ; return ; } }
};
}
