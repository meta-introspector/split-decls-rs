// Generated macro for stream_type_suggestor (function)
macro_rules! Depcrate_prompts_h3_streamstream_type_suggestor {
() => {
// Module: crate::prompts::h3::stream
// Provides: {"stream_type_suggestor"}
// Dependencies: {}
fn stream_type_suggestor (val : & str) -> SuggestionResult < Vec < String > > { let suggestions = [CONTROL_STREAM , PUSH_STREAM , QPACK_ENCODER , QPACK_DECODER] ; squish_suggester (& suggestions , val) }
};
}
