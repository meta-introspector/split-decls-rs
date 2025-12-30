// Generated macro for Pacer (struct)
macro_rules! Depcrate_connection_pacingPacer {
() => {
// Module: crate::connection::pacing
// Provides: {"Pacer"}
// Dependencies: {}
# [doc = " A simple token-bucket pacer"] # [doc = ""] # [doc = " The pacer's capacity is derived on a fraction of the congestion window"] # [doc = " which can be sent in regular intervals"] # [doc = " Once the bucket is empty, further transmission is blocked."] # [doc = " The bucket refills at a rate slightly faster"] # [doc = " than one congestion window per RTT, as recommended in"] # [doc = " <https://tools.ietf.org/html/draft-ietf-quic-recovery-34#section-7.7>"] pub (super) struct Pacer { capacity : u64 , last_window : u64 , last_mtu : u16 , tokens : u64 , prev : Instant , }
};
}
