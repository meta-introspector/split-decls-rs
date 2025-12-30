// Generated macro for MIN_AUTOMATIC_ACK_DELAY (const)
macro_rules! Depcrate_connection_ack_frequencyMIN_AUTOMATIC_ACK_DELAY {
() => {
// Module: crate::connection::ack_frequency
// Provides: {"MIN_AUTOMATIC_ACK_DELAY"}
// Dependencies: {}
# [doc = " Minimum value to request the peer set max ACK delay to when the peer supports the ACK frequency"] # [doc = " extension and an explicit max ACK delay is not configured."] const MIN_AUTOMATIC_ACK_DELAY : Duration = Duration :: from_millis (25) ;
};
}
