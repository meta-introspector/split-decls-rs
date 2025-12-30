// Generated macro for impl_186 (impl)
macro_rules! Depcrate_semaphoreimpl_186 {
() => {
// Module: crate::semaphore
// Provides: {"impl_186"}
// Dependencies: {}
impl < 'a > EventListenerFuture for AcquireInner < 'a > { type Output = SemaphoreGuard < 'a > ; fn poll_with_strategy < 'x , S : Strategy < 'x > > (self : Pin < & mut Self > , strategy : & mut S , cx : & mut S :: Context ,) -> Poll < Self :: Output > { let this = self . project () ; loop { match this . semaphore . try_acquire () { Some (guard) => return Poll :: Ready (guard) , None => { if this . listener . is_none () { * this . listener = Some (this . semaphore . event . listen ()) ; } else { ready ! (strategy . poll (this . listener , cx)) ; } } } } } }
};
}
