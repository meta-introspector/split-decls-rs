// Generated macro for MessageIterator (type)
macro_rules! Depcrate_messagesMessageIterator {
() => {
// Module: crate::messages
// Provides: {"MessageIterator"}
// Dependencies: {}
# [doc = " An iterator of Message."] type MessageIterator < R > = serde_json :: StreamDeserializer < 'static , serde_json :: de :: IoRead < R > , Message > ;
};
}
