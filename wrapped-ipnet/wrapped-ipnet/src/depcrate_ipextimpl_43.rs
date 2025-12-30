// Generated macro for impl_43 (impl)
macro_rules! Depcrate_ipextimpl_43 {
() => {
// Module: crate::ipext
// Provides: {"impl_43"}
// Dependencies: {}
impl DoubleEndedIterator for Ipv4AddrRange { fn next_back (& mut self) -> Option < Self :: Item > { match self . start . partial_cmp (& self . end) { Some (Less) => { let next_back = self . end . sub_one () ; Some (mem :: replace (& mut self . end , next_back)) } , Some (Equal) => { self . end . replace_zero () ; Some (self . start . replace_one ()) } , _ => None } } fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { let n = n as u64 ; let count = self . count_u64 () ; if n >= count { self . end . replace_zero () ; self . start . replace_one () ; None } else if n == count - 1 { self . end . replace_zero () ; Some (self . start . replace_one ()) } else { let nth_back = self . end . saturating_sub (n as u32) ; self . end = nth_back . sub_one () ; Some (nth_back) } } }
};
}
