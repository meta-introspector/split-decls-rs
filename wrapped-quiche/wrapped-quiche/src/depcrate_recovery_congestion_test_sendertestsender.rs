// Generated macro for TestSender (struct)
macro_rules! Depcrate_recovery_congestion_test_senderTestSender {
() => {
// Module: crate::recovery::congestion::test_sender
// Provides: {"TestSender"}
// Dependencies: {}
pub (crate) struct TestSender { cc : Congestion , pub (crate) next_pkt : u64 , pub (crate) next_ack : u64 , pub (crate) bytes_in_flight : usize , pub (crate) time : Instant , rtt_stats : RttStats , sent_packets : VecDeque < Sent > , }
};
}
