// Generated macro for default_reply (function)
macro_rules! Depcrate_channeldefault_reply {
() => {
// Module: crate::channel
// Provides: {"default_reply"}
// Dependencies: {}
# [doc = " Handles what we need to be a good D-Bus citizen."] # [doc = ""] # [doc = " Call this if you have not handled the message yourself:"] # [doc = " * It handles calls to org.freedesktop.DBus.Peer."] # [doc = " * For other method calls, it sends an error reply back that the method was unknown."] pub fn default_reply (m : & Message) -> Option < Message > { peer (& m) . or_else (| | unknown_method (& m)) }
};
}
