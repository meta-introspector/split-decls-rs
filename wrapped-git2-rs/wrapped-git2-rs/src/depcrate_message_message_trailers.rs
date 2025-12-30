// Generated macro for _message_trailers (function)
macro_rules! Depcrate_message_message_trailers {
() => {
// Module: crate::message
// Provides: {"_message_trailers"}
// Dependencies: {}
fn _message_trailers (message : CString) -> Result < MessageTrailers , Error > { let ret = MessageTrailers :: new () ; unsafe { try_call ! (raw :: git_message_trailers (ret . raw () , message)) ; } Ok (ret) }
};
}
