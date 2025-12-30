// Generated macro for impl_40 (impl)
macro_rules! Depcrate_file_formats_v9impl_40 {
() => {
// Module: crate::file_formats::v9
// Provides: {"impl_40"}
// Dependencies: {}
impl super :: EventDecoder for EventDecoder { fn num_events (& self) -> usize { self . num_events () } fn metadata (& self) -> Metadata { self . metadata () } fn decode_full_event (& self , event_index : usize) -> Event < '_ > { self . decode_full_event (event_index) } fn decode_lightweight_event (& self , event_index : usize) -> LightweightEvent { self . decode_lightweight_event (event_index) } }
};
}
