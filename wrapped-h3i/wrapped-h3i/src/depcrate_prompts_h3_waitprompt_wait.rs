// Generated macro for prompt_wait (function)
macro_rules! Depcrate_prompts_h3_waitprompt_wait {
() => {
// Module: crate::prompts::h3::wait
// Provides: {"prompt_wait"}
// Dependencies: {}
pub fn prompt_wait () -> InquireResult < Action > { let wait_type = Text :: new ("wait type:") . with_autocomplete (& wait_type_suggestor) . with_validator (wait_type_validator) . prompt () ? ; let actual = match wait_type . as_str () { DURATION => Some (prompt_wait_period ()) , t @ (HEADERS | DATA | FINISHED) => Some (prompt_stream_wait (t)) , _ => None , } ; let action = Action :: Wait { wait_type : actual . unwrap () ? , } ; Ok (action) }
};
}
