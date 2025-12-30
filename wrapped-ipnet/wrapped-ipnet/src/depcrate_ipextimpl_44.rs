// Generated macro for impl_44 (impl)
macro_rules! Depcrate_ipextimpl_44 {
() => {
// Module: crate::ipext
// Provides: {"impl_44"}
// Dependencies: {}
impl DoubleEndedIterator for Ipv6AddrRange { fn next_back (& mut self) -> Option < Self :: Item > { match self . start . partial_cmp (& self . end) { Some (Less) => { let next_back = self . end . sub_one () ; Some (mem :: replace (& mut self . end , next_back)) } , Some (Equal) => { self . end . replace_zero () ; Some (self . start . replace_one ()) } , _ => None } } fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { let n = n as u128 ; if self . can_count_u128 () { let count = self . count_u128 () ; if n >= count { self . end . replace_zero () ; self . start . replace_one () ; None } else if n == count - 1 { self . end . replace_zero () ; Some (self . start . replace_one ()) } else { let nth_back = self . end . saturating_sub (n) ; self . end = nth_back . sub_one () ; Some (nth_back) } } else { let nth_back = self . end . saturating_sub (n) ; self . end = nth_back . sub_one () ; Some (nth_back) } } }
};
}
