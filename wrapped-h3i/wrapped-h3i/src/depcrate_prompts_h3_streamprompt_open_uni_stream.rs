// Generated macro for prompt_open_uni_stream (function)
macro_rules! Depcrate_prompts_h3_streamprompt_open_uni_stream {
() => {
// Module: crate::prompts::h3::stream
// Provides: {"prompt_open_uni_stream"}
// Dependencies: {}
pub fn prompt_open_uni_stream (sid_alloc : & mut StreamIdAllocator ,) -> InquireResult < Action > { let stream_id = autopick_stream_id (sid_alloc) ? ; let stream_type = Text :: new ("stream type:") . with_validator (validate_stream_type) . with_autocomplete (& stream_type_suggestor) . prompt () ? ; let ty = match stream_type . as_str () { CONTROL_STREAM => 0x0 , PUSH_STREAM => 0x1 , QPACK_ENCODER => 0x2 , QPACK_DECODER => 0x3 , _ => stream_type . parse :: < u64 > () . unwrap () , } ; let fin_stream = prompt_fin_stream () ? ; Ok (Action :: OpenUniStream { stream_id , fin_stream , stream_type : ty , }) }
};
}
