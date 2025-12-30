// Generated macro for optimal_capacity (function)
macro_rules! Depcrate_connection_pacingoptimal_capacity {
() => {
// Module: crate::connection::pacing
// Provides: {"optimal_capacity"}
// Dependencies: {}
# [doc = " Calculates a pacer capacity for a certain window and RTT"] # [doc = ""] # [doc = " The goal is to emit a burst (of size `capacity`) in timer intervals"] # [doc = " which compromise between"] # [doc = " - ideally distributing datagrams over time"] # [doc = " - constantly waking up the connection to produce additional datagrams"] # [doc = ""] # [doc = " Too short burst intervals means we will never meet them since the timer"] # [doc = " accuracy in user-space is not high enough. If we miss the interval by more"] # [doc = " than 25%, we will lose that part of the congestion window since no additional"] # [doc = " tokens for the extra-elapsed time can be stored."] # [doc = ""] # [doc = " Too long burst intervals make pacing less effective."] fn optimal_capacity (smoothed_rtt : Duration , window : u64 , mtu : u16) -> u64 { let rtt = smoothed_rtt . as_nanos () . max (1) ; let mtu = u64 :: from (mtu) ; let target_capacity = ((window as u128 * TARGET_BURST_INTERVAL . as_nanos ()) / rtt) as u64 ; let max_capacity = Ord :: max (((window as u128 * MAX_BURST_INTERVAL . as_nanos ()) / rtt) as u64 , mtu ,) ; Ord :: min (max_capacity , target_capacity . clamp (MIN_BURST_SIZE * mtu , MAX_BURST_SIZE * mtu) ,) }
};
}
