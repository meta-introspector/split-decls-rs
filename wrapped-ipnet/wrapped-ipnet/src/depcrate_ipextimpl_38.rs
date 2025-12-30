// Generated macro for impl_38 (impl)
macro_rules! Depcrate_ipextimpl_38 {
() => {
// Module: crate::ipext
// Provides: {"impl_38"}
// Dependencies: {}
impl Ipv6AddrRange { pub fn new (start : Ipv6Addr , end : Ipv6Addr) -> Self { Ipv6AddrRange { start : start , end : end , } } # [doc = " Counts the number of Ipv6Addr in this range."] # [doc = " This method may overflow or panic if start"] # [doc = " is 0 and end is u128::MAX"] fn count_u128 (& self) -> u128 { match self . start . partial_cmp (& self . end) { Some (Less) => { let count = self . end . saturating_sub (self . start) ; count + 1 } , Some (Equal) => 1 , _ => 0 , } } # [doc = " True only if count_u128 does not overflow"] fn can_count_u128 (& self) -> bool { self . start != Ipv6Addr :: new (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) || self . end != Ipv6Addr :: new (0xffff , 0xffff , 0xffff , 0xffff , 0xffff , 0xffff , 0xffff , 0xffff) } }
};
}
