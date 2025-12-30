// Generated macro for BytesInFlight (struct)
macro_rules! Depcrate_recovery_bytes_in_flightBytesInFlight {
() => {
// Module: crate::recovery::bytes_in_flight
// Provides: {"BytesInFlight"}
// Dependencies: {}
# [doc = " Estimate the total duration a connection has bytes-in-flight."] # [doc = ""] # [doc = " There can be multiple transitions from bytes-in-flight >0 to 0 and 0 to >0"] # [doc = " during a connection's lifetime. Total bytes-in-flight duration is the sum of"] # [doc = " all intervals that transition from idle to not-idle and back to idle. Close"] # [doc = " intervals are the ones that transitioned back to idle. The open one is the"] # [doc = " most recent interval for which we only have a start time, but no end time."] # [derive (Default)] pub struct BytesInFlight { bytes_in_flight : usize , bytes_in_flight_interval_start : Option < Instant > , open_interval_duration : Duration , closed_interval_duration : Duration , }
};
}
