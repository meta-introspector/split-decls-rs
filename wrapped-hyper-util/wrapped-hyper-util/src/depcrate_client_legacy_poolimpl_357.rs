// Generated macro for impl_357 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_357 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_357"}
// Dependencies: {}
impl < T : Poolable , K : Key > PoolInner < T , K > { # [doc = " This should *only* be called by the IdleTask"] fn clear_expired (& mut self) { let dur = self . timeout . expect ("interval assumes timeout") ; let now = self . now () ; self . idle . retain (| key , values | { values . retain (| entry | { if ! entry . value . is_open () { trace ! ("idle interval evicting closed for {:?}" , key) ; return false ; } if now . saturating_duration_since (entry . idle_at) > dur { trace ! ("idle interval evicting expired for {:?}" , key) ; return false ; } true }) ; ! values . is_empty () }) ; } }
};
}
