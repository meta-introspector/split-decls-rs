// Generated macro for Pacer (struct)
macro_rules! Depcrate_recovery_congestion_pacerPacer {
() => {
// Module: crate::recovery::congestion::pacer
// Provides: {"Pacer"}
// Dependencies: {}
# [derive (Debug)] pub struct Pacer { # [doc = " Whether pacing is enabled."] enabled : bool , # [doc = " Bucket capacity (bytes)."] capacity : usize , # [doc = " Bucket used (bytes)."] used : usize , # [doc = " Sending pacing rate (bytes/sec)."] rate : u64 , # [doc = " Timestamp of the last packet sent time update."] last_update : Instant , # [doc = " Timestamp of the next packet to be sent."] next_time : Instant , # [doc = " Current MSS."] max_datagram_size : usize , # [doc = " Last packet size."] last_packet_size : Option < usize > , # [doc = " Interval to be added in next burst."] iv : Duration , # [doc = " Max pacing rate (bytes/sec)."] max_pacing_rate : Option < u64 > , }
};
}
