// Generated macro for prompt_connection_close (function)
macro_rules! Depcrate_prompts_h3prompt_connection_close {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_connection_close"}
// Dependencies: {}
pub fn prompt_connection_close () -> InquireResult < Action > { let (error_space , error_code) = errors :: prompt_transport_or_app_error () ? ; let reason = Text :: new ("reason phrase:") . with_placeholder ("optional reason phrase") . prompt () . unwrap_or_default () ; Ok (Action :: ConnectionClose { error : ConnectionError { is_app : matches ! (error_space , ErrorSpace :: ApplicationError) , error_code , reason : reason . as_bytes () . to_vec () , } , }) }
};
}
