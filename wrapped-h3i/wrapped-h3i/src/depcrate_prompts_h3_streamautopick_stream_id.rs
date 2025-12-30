// Generated macro for autopick_stream_id (function)
macro_rules! Depcrate_prompts_h3_streamautopick_stream_id {
() => {
// Module: crate::prompts::h3::stream
// Provides: {"autopick_stream_id"}
// Dependencies: {}
pub fn autopick_stream_id (sid_alloc : & mut StreamIdAllocator ,) -> InquireResult < u64 > { let stream_id = Text :: new (STREAM_ID_PROMPT) . with_placeholder (EMPTY_PICKS) . with_help_message (ESC_TO_RET) . with_validator (validate_stream_id) . prompt () ? ; Ok (match stream_id . as_str () { "" => { let id = sid_alloc . take_next_id () ; println ! ("{AUTO_PICK}={id}") ; id } , _ => stream_id . parse :: < u64 > () . unwrap () , }) }
};
}
