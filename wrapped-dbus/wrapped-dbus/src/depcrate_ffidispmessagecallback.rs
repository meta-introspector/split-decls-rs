// Generated macro for MessageCallback (type)
macro_rules! Depcrate_ffidispMessageCallback {
() => {
// Module: crate::ffidisp
// Provides: {"MessageCallback"}
// Dependencies: {}
# [doc = " The type of function to use for replacing the message callback."] # [doc = ""] # [doc = " See the documentation for Connection::replace_message_callback for more information."] pub type MessageCallback = Box < dyn FnMut (& Connection , Message) -> bool + 'static > ;
};
}
