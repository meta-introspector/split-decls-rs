// Generated macro for v8_lightweightevent_as_current (function)
macro_rules! Depcrate_file_formats_v8v8_lightweightevent_as_current {
() => {
// Module: crate::file_formats::v8
// Provides: {"v8_lightweightevent_as_current"}
// Dependencies: {}
fn v8_lightweightevent_as_current (old : OldLightweightEvent) -> LightweightEvent { LightweightEvent { event_index : old . event_index , thread_id : old . thread_id , payload : v8_event_payload_as_current (old . payload) , } }
};
}
