// Generated macro for bbr_bytes_in_net (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_bytes_in_net {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_bytes_in_net"}
// Dependencies: {}
# [doc = " Estimates the number of bytes \"in the network\" based on pacing."] # [doc = ""] # [doc = " When pacing is implemented at lower layers (e.g. when TX_TIME and the sch_fq"] # [doc = " qdisc are used), the in-flight data can be higher than the amount data"] # [doc = " actually sent on the network."] # [doc = ""] # [doc = " This is largely based on the [`bbr_packets_in_net_at_edt()`] function from"] # [doc = " Linux' BBR implementation."] # [doc = ""] # [doc = " [`bbr_packets_in_net_at_edt()`]: https://elixir.bootlin.com/linux/v6.13.7/source/net/ipv4/tcp_bbr.c#L437"] fn bbr_bytes_in_net (r : & Congestion , in_flight : usize , now : Instant) -> usize { let edt = r . pacer . next_time () . max (now) ; let interval = edt . saturating_duration_since (now) ; let interval_delivered = interval . as_secs_f64 () * r . bbr_state . btlbw as f64 ; let mut in_flight_at_edt = in_flight ; if r . bbr_state . pacing_gain > 1.0 { in_flight_at_edt += r . send_quantum ; } in_flight_at_edt . saturating_sub (interval_delivered as usize) }
};
}
