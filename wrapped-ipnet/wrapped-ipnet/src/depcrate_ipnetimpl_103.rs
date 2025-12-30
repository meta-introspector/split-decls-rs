// Generated macro for impl_103 (impl)
macro_rules! Depcrate_ipnetimpl_103 {
() => {
// Module: crate::ipnet
// Provides: {"impl_103"}
// Dependencies: {}
impl Iterator for Ipv4Subnets { type Item = Ipv4Net ; fn next (& mut self) -> Option < Self :: Item > { match self . start . partial_cmp (& self . end) { Some (Less) => { let next = next_ipv4_subnet (self . start , self . end , self . min_prefix_len) ; self . start = next . broadcast () . saturating_add (1) ; if self . start == next . broadcast () { self . end . replace_zero () ; } Some (next) } , Some (Equal) => { let next = next_ipv4_subnet (self . start , self . end , self . min_prefix_len) ; self . start = next . broadcast () . saturating_add (1) ; self . end . replace_zero () ; Some (next) } , _ => None , } } }
};
}
