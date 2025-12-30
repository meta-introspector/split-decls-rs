// Generated macro for msg_send_bool (macro)
macro_rules! Depcrate___macros_msg_sendmsg_send_bool {
() => {
// Module: crate::__macros::msg_send
// Provides: {"msg_send_bool"}
// Dependencies: {}
# [doc = " Use [`msg_send!`] instead, it now supports converting to/from `bool`."] # [macro_export] # [deprecated = "use a normal msg_send! instead, it will perform the conversion for you"] macro_rules ! msg_send_bool { [$ ($ msg_send_args : tt) +] => ({ let result : $ crate :: runtime :: Bool = $ crate :: msg_send ! [$ ($ msg_send_args) +] ; result . as_bool () }) ; }
};
}
