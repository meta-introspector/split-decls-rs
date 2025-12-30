// Generated macro for impl_48 (impl)
macro_rules! Depcrate_muteximpl_48 {
() => {
// Module: crate::mutex
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a , T : ? Sized > EventListenerFuture for LockInner < 'a , T > { type Output = MutexGuard < 'a , T > ; # [inline] fn poll_with_strategy < 'x , S : event_listener_strategy :: Strategy < 'x > > (self : Pin < & mut Self > , strategy : & mut S , context : & mut S :: Context ,) -> Poll < Self :: Output > { let mut this = self . project () ; if this . acquire_slow . is_none () { match this . mutex . try_lock () { Some (guard) => return Poll :: Ready (guard) , None => { this . acquire_slow . set (Some (AcquireSlow :: new (this . mutex))) ; } } } ready ! (this . acquire_slow . as_pin_mut () . unwrap () . poll_with_strategy (strategy , context)) ; Poll :: Ready (MutexGuard (this . mutex)) } }
};
}
