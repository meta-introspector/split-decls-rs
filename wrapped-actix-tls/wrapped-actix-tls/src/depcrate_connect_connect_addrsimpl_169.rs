// Generated macro for impl_169 (impl)
macro_rules! Depcrate_connect_connect_addrsimpl_169 {
() => {
// Module: crate::connect::connect_addrs
// Provides: {"impl_169"}
// Dependencies: {}
impl Iterator for ConnectAddrsIter < '_ > { type Item = SocketAddr ; fn next (& mut self) -> Option < Self :: Item > { match * self { Self :: None => None , Self :: One (addr) => { * self = Self :: None ; Some (addr) } Self :: Multi (ref mut iter) => iter . next () . copied () , Self :: MultiOwned (ref mut iter) => iter . next () , } } fn size_hint (& self) -> (usize , Option < usize >) { match * self { Self :: None => (0 , Some (0)) , Self :: One (_) => (1 , Some (1)) , Self :: Multi (ref iter) => iter . size_hint () , Self :: MultiOwned (ref iter) => iter . size_hint () , } } }
};
}
