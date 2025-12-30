// Generated macro for v8_event_payload_as_current (function)
macro_rules! Depcrate_file_formats_v8v8_event_payload_as_current {
() => {
// Module: crate::file_formats::v8
// Provides: {"v8_event_payload_as_current"}
// Dependencies: {}
fn v8_event_payload_as_current (old : OldEventPayload) -> EventPayload { match old { OldEventPayload :: Timestamp (t) => EventPayload :: Timestamp (v8_timestamp_as_current (t)) , OldEventPayload :: Integer (t) => EventPayload :: Integer (t) , } }
};
}
