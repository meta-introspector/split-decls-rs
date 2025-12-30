// Generated macro for MessageRingBuffer (struct)
macro_rules! Depcrate_messagesMessageRingBuffer {
() => {
// Module: crate::messages
// Provides: {"MessageRingBuffer"}
// Dependencies: {}
# [doc = " A ring buffer for messages."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct MessageRingBuffer { pub (crate) buf : Vec < Message > , cursor : usize , total : usize , }
};
}
