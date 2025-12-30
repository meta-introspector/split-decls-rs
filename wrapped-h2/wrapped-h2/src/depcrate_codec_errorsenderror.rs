// Generated macro for SendError (enum)
macro_rules! Depcrate_codec_errorSendError {
() => {
// Module: crate::codec::error
// Provides: {"SendError"}
// Dependencies: {}
# [doc = " Errors caused by sending a message"] # [derive (Debug)] pub enum SendError { Connection (Error) , User (UserError) , }
};
}
