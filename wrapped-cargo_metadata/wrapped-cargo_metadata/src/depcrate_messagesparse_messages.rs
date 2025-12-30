// Generated macro for parse_messages (function)
macro_rules! Depcrate_messagesparse_messages {
() => {
// Module: crate::messages
// Provides: {"parse_messages"}
// Dependencies: {}
# [doc = " Creates an iterator of Message from a Read outputting a stream of JSON"] # [doc = " messages. For usage information, look at the top-level documentation."] # [deprecated (note = "Use Message::parse_stream instead")] pub fn parse_messages < R : Read > (input : R) -> MessageIterator < R > { serde_json :: Deserializer :: from_reader (input) . into_iter :: < Message > () }
};
}
