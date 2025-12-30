// Generated macro for __msg_send_helper (macro)
macro_rules! Depcrate___macros_msg_send__msg_send_helper {
() => {
// Module: crate::__macros::msg_send
// Provides: {"__msg_send_helper"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __msg_send_helper { { ($ ($ fn_args : tt) +) ($ trait : ident :: $ fn : ident) ($ ($ selector : tt) *) ($ ($ argument : expr ,) *) } => ({ let result ; result = <$ crate :: __method_family ! (() ($ ($ selector) *)) as $ crate :: __macros ::$ trait < _ , _ >>::$ fn ($ ($ fn_args) +, $ crate :: sel ! ($ ($ selector) *) , ($ ($ argument ,) *) ,) ; result }) ; }
};
}
