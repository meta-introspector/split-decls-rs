// Generated macro for impl_227 (impl)
macro_rules! Depcrate_utils_channelimpl_227 {
() => {
// Module: crate::utils::channel
// Provides: {"impl_227"}
// Dependencies: {}
impl < T > Stream for LocalReceiver < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut channel = self . channel . borrow_mut () ; match channel . queue . pop_front () { Some (item) => Poll :: Ready (Some (item)) , None => { if channel . closed { Poll :: Ready (None) } else { match & mut channel . waker { Some (prev) => prev . clone_from (cx . waker ()) , None => channel . waker = Some (cx . waker () . clone ()) , } Poll :: Pending } } } } }
};
}
