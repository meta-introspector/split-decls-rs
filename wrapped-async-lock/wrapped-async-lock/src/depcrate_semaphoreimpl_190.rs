// Generated macro for impl_190 (impl)
macro_rules! Depcrate_semaphoreimpl_190 {
() => {
// Module: crate::semaphore
// Provides: {"impl_190"}
// Dependencies: {}
impl EventListenerFuture for AcquireArcInner { type Output = SemaphoreGuardArc ; fn poll_with_strategy < 'x , S : Strategy < 'x > > (self : Pin < & mut Self > , strategy : & mut S , cx : & mut S :: Context ,) -> Poll < Self :: Output > { let this = self . project () ; loop { match this . semaphore . try_acquire_arc () { Some (guard) => return Poll :: Ready (guard) , None => { if this . listener . is_none () { * this . listener = Some (this . semaphore . event . listen ()) ; } else { ready ! (strategy . poll (this . listener , cx)) ; } } } } } }
};
}
