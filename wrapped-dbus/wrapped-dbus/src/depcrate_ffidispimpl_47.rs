// Generated macro for impl_47 (impl)
macro_rules! Depcrate_ffidispimpl_47 {
() => {
// Module: crate::ffidisp
// Provides: {"impl_47"}
// Dependencies: {}
impl MsgHandlerType { fn matches_msg (& self , m : & Message) -> bool { match * self { MsgHandlerType :: All => true , MsgHandlerType :: MsgType (t) => m . msg_type () == t , MsgHandlerType :: Reply (serial) => { let t = m . msg_type () ; ((t == MessageType :: MethodReturn) || (t == MessageType :: Error)) && (m . get_reply_serial () == Some (serial)) } } } }
};
}
