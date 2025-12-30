// Generated macro for impl_79 (impl)
macro_rules! Depcrate_utils_poll_state_maybe_doneimpl_79 {
() => {
// Module: crate::utils::poll_state::maybe_done
// Provides: {"impl_79"}
// Dependencies: {}
impl < Fut : Future > Future for MaybeDone < Fut > { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let res = unsafe { match Pin :: as_mut (& mut self) . get_unchecked_mut () { MaybeDone :: Future (a) => ready ! (Pin :: new_unchecked (a) . poll (cx)) , MaybeDone :: Done (_) => return Poll :: Ready (()) , MaybeDone :: Gone => panic ! ("MaybeDone polled after value taken") , } } ; self . set (MaybeDone :: Done (res)) ; Poll :: Ready (()) } }
};
}
