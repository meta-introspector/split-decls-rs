// Generated macro for AckFrequencyState (struct)
macro_rules! Depcrate_connection_ack_frequencyAckFrequencyState {
() => {
// Module: crate::connection::ack_frequency
// Provides: {"AckFrequencyState"}
// Dependencies: {}
# [doc = " State associated to ACK frequency"] pub (super) struct AckFrequencyState { in_flight_ack_frequency_frame : Option < (u64 , Duration) > , next_outgoing_sequence_number : VarInt , pub (super) peer_max_ack_delay : Duration , last_ack_frequency_frame : Option < u64 > , pub (super) max_ack_delay : Duration , }
};
}
