// Generated macro for Timestamp (enum)
macro_rules! Depcrate_event_payloadTimestamp {
() => {
// Module: crate::event_payload
// Provides: {"Timestamp"}
// Dependencies: {}
# [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub enum Timestamp { Interval { start : SystemTime , end : SystemTime } , Instant (SystemTime) , }
};
}
