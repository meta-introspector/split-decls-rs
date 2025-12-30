// Generated macro for QUIC_KEEP_ALIVE (const)
macro_rules! DepcrateQUIC_KEEP_ALIVE {
() => {
// Module: crate
// Provides: {"QUIC_KEEP_ALIVE"}
// Dependencies: {}
# [doc = " To avoid idle timeout, the QUIC endpoint sends a ping every"] # [doc = " QUIC_KEEP_ALIVE. This shouldn't be too low to avoid unnecessary ping traffic."] pub const QUIC_KEEP_ALIVE : Duration = Duration :: from_secs (45) ;
};
}
