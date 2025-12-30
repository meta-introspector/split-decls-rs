// Generated macro for Pacer (struct)
macro_rules! Depcrate_recovery_gcongestion_pacerPacer {
() => {
// Module: crate::recovery::gcongestion::pacer
// Provides: {"Pacer"}
// Dependencies: {}
# [derive (Debug)] pub struct Pacer { # [doc = " Should this [`Pacer`] be making any release decisions?"] enabled : bool , # [doc = " Underlying sender"] sender : BBRv2 , # [doc = " The maximum rate the [`Pacer`] will use."] max_pacing_rate : Option < Bandwidth > , # [doc = " Number of unpaced packets to be sent before packets are delayed."] burst_tokens : usize , # [doc = " When can the next packet be sent."] ideal_next_packet_send_time : ReleaseTime , initial_burst_size : usize , # [doc = " Number of unpaced packets to be sent before packets are delayed. This"] # [doc = " token is consumed after [`Self::burst_tokens`] ran out."] lumpy_tokens : usize , # [doc = " Indicates whether pacing throttles the sending. If true, make up for"] # [doc = " lost time."] pacing_limited : bool , }
};
}
