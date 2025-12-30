// Generated macro for __extern_protocol_check_no_derives (macro)
macro_rules! Depcrate___macros_extern_protocol__extern_protocol_check_no_derives {
() => {
// Module: crate::__macros::extern_protocol
// Provides: {"__extern_protocol_check_no_derives"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __extern_protocol_check_no_derives { () => { } ; ($ ($ ivars : tt) *) => { $ crate :: __macros :: compile_error ! ("#[derive(...)] is not supported in extern_protocol!") ; } ; }
};
}
