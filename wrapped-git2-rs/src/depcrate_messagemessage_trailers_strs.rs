// Generated macro for message_trailers_strs (function)
macro_rules! Depcrate_messagemessage_trailers_strs {
() => {
// Module: crate::message
// Provides: {"message_trailers_strs"}
// Dependencies: {}
# [doc = " Get the trailers for the given message."] # [doc = ""] # [doc = " Use this function when you are dealing with a UTF-8-encoded message."] pub fn message_trailers_strs (message : & str) -> Result < MessageTrailersStrs , Error > { _message_trailers (message . into_c_string () ?) . map (| res | MessageTrailersStrs (res)) }
};
}
