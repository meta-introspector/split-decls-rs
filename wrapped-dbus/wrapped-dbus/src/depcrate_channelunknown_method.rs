// Generated macro for unknown_method (function)
macro_rules! Depcrate_channelunknown_method {
() => {
// Module: crate::channel
// Provides: {"unknown_method"}
// Dependencies: {}
# [doc = " For method calls, it replies that the method was unknown, otherwise returns None."] fn unknown_method (m : & Message) -> Option < Message > { if m . msg_type () != MessageType :: MethodCall { return None ; } Some (m . error (& "org.freedesktop.DBus.Error.UnknownMethod" . into () , & to_c_str ("Path, Interface, or Method does not exist"))) }
};
}
