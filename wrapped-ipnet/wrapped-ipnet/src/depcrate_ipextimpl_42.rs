// Generated macro for impl_42 (impl)
macro_rules! Depcrate_ipextimpl_42 {
() => {
// Module: crate::ipext
// Provides: {"impl_42"}
// Dependencies: {}
impl DoubleEndedIterator for IpAddrRange { fn next_back (& mut self) -> Option < Self :: Item > { match * self { IpAddrRange :: V4 (ref mut a) => a . next_back () . map (IpAddr :: V4) , IpAddrRange :: V6 (ref mut a) => a . next_back () . map (IpAddr :: V6) , } } fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { match * self { IpAddrRange :: V4 (ref mut a) => a . nth_back (n) . map (IpAddr :: V4) , IpAddrRange :: V6 (ref mut a) => a . nth_back (n) . map (IpAddr :: V6) , } } }
};
}
