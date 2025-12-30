// Generated macro for MessageCopyState (struct)
macro_rules! Depcrate_messagesMessageCopyState {
() => {
// Module: crate::messages
// Provides: {"MessageCopyState"}
// Dependencies: {}
# [doc = " State used to keep track of what's new since the last time message were copied."] # [doc = ""] # [doc = " Note that due to the nature of a ring buffer, there is no guarantee that you see all messages."] pub struct MessageCopyState { cursor : usize , buf_len : usize , total : usize , }
};
}
