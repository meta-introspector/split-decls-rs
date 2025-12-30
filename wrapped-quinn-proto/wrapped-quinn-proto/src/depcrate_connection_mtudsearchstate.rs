// Generated macro for SearchState (struct)
macro_rules! Depcrate_connection_mtudSearchState {
() => {
// Module: crate::connection::mtud
// Provides: {"SearchState"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] struct SearchState { # [doc = " The lower bound for the current binary search"] lower_bound : u16 , # [doc = " The upper bound for the current binary search"] upper_bound : u16 , # [doc = " The minimum change to stop the current binary search"] minimum_change : u16 , # [doc = " The UDP payload size we last sent a probe for"] last_probed_mtu : u16 , # [doc = " Packet number of an in-flight probe (if any)"] in_flight_probe : Option < u64 > , # [doc = " Lost probes at the current probe size"] lost_probe_count : usize , }
};
}
