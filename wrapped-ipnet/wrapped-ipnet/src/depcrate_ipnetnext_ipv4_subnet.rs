// Generated macro for next_ipv4_subnet (function)
macro_rules! Depcrate_ipnetnext_ipv4_subnet {
() => {
// Module: crate::ipnet
// Provides: {"next_ipv4_subnet"}
// Dependencies: {}
fn next_ipv4_subnet (start : Ipv4Addr , end : Ipv4Addr , min_prefix_len : u8) -> Ipv4Net { let range = end . saturating_sub (start) . saturating_add (1) ; if range == core :: u32 :: MAX && min_prefix_len == 0 { Ipv4Net :: new (start , min_prefix_len) . unwrap () } else { let range_bits = 32u32 . saturating_sub (range . leading_zeros ()) . saturating_sub (1) ; let start_tz = u32 :: from (start) . trailing_zeros () ; let new_prefix_len = 32 - min (range_bits , start_tz) ; let next_prefix_len = max (new_prefix_len as u8 , min_prefix_len) ; Ipv4Net :: new (start , next_prefix_len) . unwrap () } }
};
}
