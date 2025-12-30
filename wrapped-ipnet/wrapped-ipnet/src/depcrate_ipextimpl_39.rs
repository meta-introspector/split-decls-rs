// Generated macro for impl_39 (impl)
macro_rules! Depcrate_ipextimpl_39 {
() => {
// Module: crate::ipext
// Provides: {"impl_39"}
// Dependencies: {}
impl Iterator for IpAddrRange { type Item = IpAddr ; fn next (& mut self) -> Option < Self :: Item > { match * self { IpAddrRange :: V4 (ref mut a) => a . next () . map (IpAddr :: V4) , IpAddrRange :: V6 (ref mut a) => a . next () . map (IpAddr :: V6) , } } fn count (self) -> usize { match self { IpAddrRange :: V4 (a) => a . count () , IpAddrRange :: V6 (a) => a . count () , } } fn last (self) -> Option < Self :: Item > { match self { IpAddrRange :: V4 (a) => a . last () . map (IpAddr :: V4) , IpAddrRange :: V6 (a) => a . last () . map (IpAddr :: V6) , } } fn max (self) -> Option < Self :: Item > { match self { IpAddrRange :: V4 (a) => Iterator :: max (a) . map (IpAddr :: V4) , IpAddrRange :: V6 (a) => Iterator :: max (a) . map (IpAddr :: V6) , } } fn min (self) -> Option < Self :: Item > { match self { IpAddrRange :: V4 (a) => Iterator :: min (a) . map (IpAddr :: V4) , IpAddrRange :: V6 (a) => Iterator :: min (a) . map (IpAddr :: V6) , } } fn nth (& mut self , n : usize) -> Option < Self :: Item > { match * self { IpAddrRange :: V4 (ref mut a) => a . nth (n) . map (IpAddr :: V4) , IpAddrRange :: V6 (ref mut a) => a . nth (n) . map (IpAddr :: V6) , } } fn size_hint (& self) -> (usize , Option < usize >) { match * self { IpAddrRange :: V4 (ref a) => a . size_hint () , IpAddrRange :: V6 (ref a) => a . size_hint () , } } }
};
}
