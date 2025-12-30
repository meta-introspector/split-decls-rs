// Generated macro for QUIC_MAX_TIMEOUT (const)
macro_rules! DepcrateQUIC_MAX_TIMEOUT {
() => {
// Module: crate
// Provides: {"QUIC_MAX_TIMEOUT"}
// Dependencies: {}
# [doc = " QUIC connection idle timeout. The connection will be closed if"] # [doc = " there are no activities on it within the timeout window."] pub const QUIC_MAX_TIMEOUT : Duration = Duration :: from_secs (60) ;
};
}
