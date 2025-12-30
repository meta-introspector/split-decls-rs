// Generated macro for get_max_ack_delay (function)
macro_rules! Depcrate_connectionget_max_ack_delay {
() => {
// Module: crate::connection
// Provides: {"get_max_ack_delay"}
// Dependencies: {}
fn get_max_ack_delay (params : & TransportParameters) -> Duration { Duration :: from_micros (params . max_ack_delay . 0 * 1000) }
};
}
