// Generated macro for impl_491 (impl)
macro_rules! Depcrate_connection_statsimpl_491 {
() => {
// Module: crate::connection::stats
// Provides: {"impl_491"}
// Dependencies: {}
impl UdpStats { pub (crate) fn on_sent (& mut self , datagrams : u64 , bytes : usize) { self . datagrams += datagrams ; self . bytes += bytes as u64 ; self . ios += 1 ; } }
};
}
