// Generated macro for impl_292 (impl)
macro_rules! Depcrate_future_maybe_doneimpl_292 {
() => {
// Module: crate::future::maybe_done
// Provides: {"impl_292"}
// Dependencies: {}
impl < Fut : Future > Future for MaybeDone < Fut > { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { unsafe { match self . as_mut () . get_unchecked_mut () { Self :: Future (f) => { let res = ready ! (Pin :: new_unchecked (f) . poll (cx)) ; self . set (Self :: Done (res)) ; } Self :: Done (_) => { } Self :: Gone => panic ! ("MaybeDone polled after value taken") , } } Poll :: Ready (()) } }
};
}
