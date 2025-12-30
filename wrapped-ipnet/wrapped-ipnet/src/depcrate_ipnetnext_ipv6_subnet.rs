// Generated macro for next_ipv6_subnet (function)
macro_rules! Depcrate_ipnetnext_ipv6_subnet {
() => {
// Module: crate::ipnet
// Provides: {"next_ipv6_subnet"}
// Dependencies: {}
fn next_ipv6_subnet (start : Ipv6Addr , end : Ipv6Addr , min_prefix_len : u8) -> Ipv6Net { let range = end . saturating_sub (start) . saturating_add (1) ; if range == core :: u128 :: MAX && min_prefix_len == 0 { Ipv6Net :: new (start , min_prefix_len) . unwrap () } else { let range = end . saturating_sub (start) . saturating_add (1) ; let range_bits = 128u32 . saturating_sub (range . leading_zeros ()) . saturating_sub (1) ; let start_tz = u128 :: from (start) . trailing_zeros () ; let new_prefix_len = 128 - min (range_bits , start_tz) ; let next_prefix_len = max (new_prefix_len as u8 , min_prefix_len) ; Ipv6Net :: new (start , next_prefix_len) . unwrap () } }
};
}
