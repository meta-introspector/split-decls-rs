// Generated macro for msg_send_id (macro)
macro_rules! Depcrate___macros_msg_sendmsg_send_id {
() => {
// Module: crate::__macros::msg_send
// Provides: {"msg_send_id"}
// Dependencies: {}
# [doc = " Use [`msg_send!`] instead, it now supports converting to/from"] # [doc = " [`Retained`][crate::rc::Retained]."] # [macro_export] # [deprecated = "use a normal msg_send! instead, it will now perform the conversion to/from `Retained` for you"] macro_rules ! msg_send_id { [$ ($ msg_send_args : tt) +] => { $ crate :: msg_send ! [$ ($ msg_send_args) *] } }
};
}
