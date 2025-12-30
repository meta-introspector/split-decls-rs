// Generated macro for impl_76 (impl)
macro_rules! Depcrate_messagesimpl_76 {
() => {
// Module: crate::messages
// Provides: {"impl_76"}
// Dependencies: {}
impl Message { # [doc = " Creates an iterator of Message from a Read outputting a stream of JSON"] # [doc = " messages. For usage information, look at the top-level documentation."] pub fn parse_stream < R : Read > (input : R) -> MessageIter < R > { MessageIter { input } } }
};
}
