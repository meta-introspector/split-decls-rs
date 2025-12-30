// Generated macro for prompt_stream_wait (function)
macro_rules! Depcrate_prompts_h3_waitprompt_stream_wait {
() => {
// Module: crate::prompts::h3::wait
// Provides: {"prompt_stream_wait"}
// Dependencies: {}
fn prompt_stream_wait (stream_wait_type : & str) -> InquireResult < WaitType > { let stream_id = prompt_stream_id () ? ; let event_type = if let HEADERS = stream_wait_type { Some (StreamEventType :: Headers) } else if let DATA = stream_wait_type { Some (StreamEventType :: Data) } else if let FINISHED = stream_wait_type { Some (StreamEventType :: Finished) } else { None } . unwrap () ; Ok (WaitType :: StreamEvent (StreamEvent { stream_id , event_type , })) }
};
}
