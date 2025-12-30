// Generated macro for impl_104 (impl)
macro_rules! Depcrate_ipnetimpl_104 {
() => {
// Module: crate::ipnet
// Provides: {"impl_104"}
// Dependencies: {}
impl Iterator for Ipv6Subnets { type Item = Ipv6Net ; fn next (& mut self) -> Option < Self :: Item > { match self . start . partial_cmp (& self . end) { Some (Less) => { let next = next_ipv6_subnet (self . start , self . end , self . min_prefix_len) ; self . start = next . broadcast () . saturating_add (1) ; if self . start == next . broadcast () { self . end . replace_zero () ; } Some (next) } , Some (Equal) => { let next = next_ipv6_subnet (self . start , self . end , self . min_prefix_len) ; self . start = next . broadcast () . saturating_add (1) ; self . end . replace_zero () ; Some (next) } , _ => None , } } }
};
}
