// Generated macro for impl_34 (impl)
macro_rules! Depcrate_file_formats_v8impl_34 {
() => {
// Module: crate::file_formats::v8
// Provides: {"impl_34"}
// Dependencies: {}
impl super :: EventDecoder for EventDecoder { fn num_events (& self) -> usize { self . num_events () } fn metadata (& self) -> Metadata { let old = self . metadata () ; v8_metadata_as_current (& old) } fn decode_full_event (& self , event_index : usize) -> Event < '_ > { let old = self . decode_full_event (event_index) ; Event { event_kind : old . event_kind , label : old . label , additional_data : old . additional_data , payload : v8_event_payload_as_current (old . payload) , thread_id : old . thread_id , } } fn decode_lightweight_event (& self , event_index : usize) -> LightweightEvent { v8_lightweightevent_as_current (self . decode_lightweight_event (event_index)) } }
};
}
