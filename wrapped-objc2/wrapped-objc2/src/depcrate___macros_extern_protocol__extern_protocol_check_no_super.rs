// Generated macro for __extern_protocol_check_no_super (macro)
macro_rules! Depcrate___macros_extern_protocol__extern_protocol_check_no_super {
() => {
// Module: crate::__macros::extern_protocol
// Provides: {"__extern_protocol_check_no_super"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __extern_protocol_check_no_super { () => { } ; ($ ($ ivars : tt) *) => { $ crate :: __macros :: compile_error ! ("#[super] is not supported in extern_protocol!") ; } ; }
};
}
