// Generated macro for impl_52 (impl)
macro_rules! Depcrate_ffidispimpl_52 {
() => {
// Module: crate::ffidisp
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'a , F : FnOnce (Result < & Message , Error >) + 'a > MsgHandler for MessageReply < F > { fn handler_type (& self) -> MsgHandlerType { MsgHandlerType :: Reply (self . 1) } fn handle_msg (& mut self , msg : & Message) -> Option < MsgHandlerResult > { let e = match msg . msg_type () { MessageType :: MethodReturn => Ok (msg) , MessageType :: Error => Err (msg . set_error_from_msg () . unwrap_err ()) , _ => unreachable ! () , } ; debug_assert_eq ! (msg . get_reply_serial () , Some (self . 1)) ; self . 0 . take () . unwrap () (e) ; return Some (MsgHandlerResult { handled : true , done : true , reply : Vec :: new () }) } }
};
}
