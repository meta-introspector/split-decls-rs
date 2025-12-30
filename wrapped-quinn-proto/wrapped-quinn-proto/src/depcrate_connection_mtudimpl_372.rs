// Generated macro for impl_372 (impl)
macro_rules! Depcrate_connection_mtudimpl_372 {
() => {
// Module: crate::connection::mtud
// Provides: {"impl_372"}
// Dependencies: {}
impl SearchState { # [doc = " Creates a new search state, with the specified lower bound (the upper bound is derived from"] # [doc = " the config and the peer's `max_udp_payload_size` transport parameter)"] fn new (mut lower_bound : u16 , peer_max_udp_payload_size : u16 , config : & MtuDiscoveryConfig ,) -> Self { lower_bound = lower_bound . min (peer_max_udp_payload_size) ; let upper_bound = config . upper_bound . clamp (lower_bound , peer_max_udp_payload_size) ; Self { in_flight_probe : None , lost_probe_count : 0 , lower_bound , upper_bound , minimum_change : config . minimum_change , last_probed_mtu : lower_bound , } } # [doc = " Determines the next MTU to probe using binary search"] fn next_mtu_to_probe (& mut self , last_probe_succeeded : bool) -> Option < u16 > { debug_assert_eq ! (self . in_flight_probe , None) ; if last_probe_succeeded { self . lower_bound = self . last_probed_mtu ; } else { self . upper_bound = self . last_probed_mtu - 1 ; } let next_mtu = (self . lower_bound as i32 + self . upper_bound as i32) / 2 ; if ((next_mtu - self . last_probed_mtu as i32) . unsigned_abs () as u16) < self . minimum_change { if self . upper_bound . saturating_sub (self . last_probed_mtu) >= self . minimum_change { return Some (self . upper_bound) ; } return None ; } Some (next_mtu as u16) } }
};
}
