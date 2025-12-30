// Generated macro for __extern_protocol_check_no_thread_kind (macro)
macro_rules! Depcrate___macros_extern_protocol__extern_protocol_check_no_thread_kind {
() => {
// Module: crate::__macros::extern_protocol
// Provides: {"__extern_protocol_check_no_thread_kind"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __extern_protocol_check_no_thread_kind { () => { } ; ($ ($ ivars : tt) *) => { $ crate :: __macros :: compile_error ! ("#[thread_kind = ...] is not supported in extern_protocol!. Add MainThreadOnly or AnyThread bound instead") ; } ; }
};
}
