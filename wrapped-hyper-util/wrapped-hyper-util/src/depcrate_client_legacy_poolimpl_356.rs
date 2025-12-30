// Generated macro for impl_356 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_356 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_356"}
// Dependencies: {}
impl < T , K : Eq + Hash > PoolInner < T , K > { # [doc = " Any `FutureResponse`s that were created will have made a `Checkout`,"] # [doc = " and possibly inserted into the pool that it is waiting for an idle"] # [doc = " connection. If a user ever dropped that future, we need to clean out"] # [doc = " those parked senders."] fn clean_waiters (& mut self , key : & K) { let mut remove_waiters = false ; if let Some (waiters) = self . waiters . get_mut (key) { waiters . retain (| tx | ! tx . is_canceled ()) ; remove_waiters = waiters . is_empty () ; } if remove_waiters { self . waiters . remove (key) ; } } }
};
}
