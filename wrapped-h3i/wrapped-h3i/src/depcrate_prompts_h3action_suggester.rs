// Generated macro for action_suggester (function)
macro_rules! Depcrate_prompts_h3action_suggester {
() => {
// Module: crate::prompts::h3
// Provides: {"action_suggester"}
// Dependencies: {}
fn action_suggester (val : & str) -> SuggestionResult < Vec < String > > { let suggestions = [HEADERS , HEADERS_NO_PSEUDO , HEADERS_LITERAL , HEADERS_NO_PSEUDO_LITERAL , DATA , SETTINGS , GOAWAY , PRIORITY_UPDATE , PUSH_PROMISE , CANCEL_PUSH , MAX_PUSH_ID , GREASE , EXTENSION , OPEN_UNI_STREAM , RESET_STREAM , STOP_SENDING , CONNECTION_CLOSE , STREAM_BYTES , DATAGRAM_QUARTER_STREAM_ID , DATAGRAM_RAW_PAYLOAD , FLUSH_PACKETS , COMMIT , WAIT , QUIT ,] ; squish_suggester (& suggestions , val) }
};
}
