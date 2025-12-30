// Generated macro for impl_54 (impl)
macro_rules! Depcrate_muteximpl_54 {
() => {
// Module: crate::mutex
// Provides: {"impl_54"}
// Dependencies: {}
impl < T : ? Sized > EventListenerFuture for LockArcInnards < T > { type Output = MutexGuardArc < T > ; fn poll_with_strategy < 'a , S : event_listener_strategy :: Strategy < 'a > > (mut self : Pin < & mut Self > , strategy : & mut S , context : & mut S :: Context ,) -> Poll < Self :: Output > { if let LockArcInnardsProj :: Unpolled { mutex } = self . as_mut () . project () { let mutex = mutex . take () . expect ("mutex taken more than once") ; if let Some (guard) = mutex . try_lock_arc () { return Poll :: Ready (guard) ; } self . as_mut () . set (LockArcInnards :: AcquireSlow { inner : AcquireSlow :: new (mutex) , }) ; } let value = match self . project () { LockArcInnardsProj :: AcquireSlow { inner } => { ready ! (inner . poll_with_strategy (strategy , context)) } _ => unreachable ! () , } ; Poll :: Ready (MutexGuardArc (value)) } }
};
}
