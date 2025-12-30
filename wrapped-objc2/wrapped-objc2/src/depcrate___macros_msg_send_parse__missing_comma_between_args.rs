// Generated macro for __missing_comma_between_args (macro)
macro_rules! Depcrate___macros_msg_send_parse__missing_comma_between_args {
() => {
// Module: crate::__macros::msg_send::parse
// Provides: {"__missing_comma_between_args"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __missing_comma_between_args { ((MsgSendSuper :: send_super_message_static) ($ ($ args : tt) *) ($ obj : expr)) => { $ crate :: __missing_comma_between_args_inner ! ("super" , $ crate :: __macros :: stringify ! (($ obj)) , $ ($ args) *) ; } ; ((MsgSendSuper :: send_super_message) ($ ($ args : tt) *) ($ obj : expr , $ superclass : expr)) => { $ crate :: __missing_comma_between_args_inner ! ("super" , $ crate :: __macros :: stringify ! (($ obj , $ superclass)) , $ ($ args) *) ; } ; ((MsgSend :: send_message) ($ ($ args : tt) *) ($ obj : expr)) => { $ crate :: __missing_comma_between_args_inner ! ($ crate :: __macros :: stringify ! ($ obj) , $ ($ args) *) ; } ; }
};
}
