// Generated macro for impl_37 (impl)
macro_rules! Depcrate_ipextimpl_37 {
() => {
// Module: crate::ipext
// Provides: {"impl_37"}
// Dependencies: {}
impl Ipv4AddrRange { pub fn new (start : Ipv4Addr , end : Ipv4Addr) -> Self { Ipv4AddrRange { start : start , end : end , } } # [doc = " Counts the number of Ipv4Addr in this range."] # [doc = " This method will never overflow or panic."] fn count_u64 (& self) -> u64 { match self . start . partial_cmp (& self . end) { Some (Less) => { let count : u32 = self . end . saturating_sub (self . start) ; let count = count as u64 + 1 ; count } , Some (Equal) => 1 , _ => 0 , } } }
};
}
