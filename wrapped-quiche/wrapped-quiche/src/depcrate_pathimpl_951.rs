// Generated macro for impl_951 (impl)
macro_rules! Depcrate_pathimpl_951 {
() => {
// Module: crate::path
// Provides: {"impl_951"}
// Dependencies: {}
impl Iterator for SocketAddrIter { type Item = SocketAddr ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let v = self . sockaddrs . get (self . index) ? ; self . index += 1 ; Some (* v) } }
};
}
