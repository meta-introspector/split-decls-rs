// Generated macro for EventDecoder (trait)
macro_rules! Depcrate_file_formatsEventDecoder {
() => {
// Module: crate::file_formats
// Provides: {"EventDecoder"}
// Dependencies: {}
# [doc = " The [EventDecoder] knows how to decode events for a specific file format."] pub trait EventDecoder : Debug + Send + Sync { fn num_events (& self) -> usize ; fn metadata (& self) -> Metadata ; fn decode_full_event < 'a > (& 'a self , event_index : usize) -> Event < 'a > ; fn decode_lightweight_event < 'a > (& 'a self , event_index : usize) -> LightweightEvent ; }
};
}
