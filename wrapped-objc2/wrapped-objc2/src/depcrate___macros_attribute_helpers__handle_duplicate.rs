// Generated macro for __handle_duplicate (macro)
macro_rules! Depcrate___macros_attribute_helpers__handle_duplicate {
() => {
// Module: crate::__macros::attribute_helpers
// Provides: {"__handle_duplicate"}
// Dependencies: {}
# [doc = " Ensure that custom attributes do not appear twice."] # [doc = ""] # [doc = " NOTE: This intentionally only results in a `compile_error!`, to allow"] # [doc = " subsequent macros to still output something (better for rust-analyzer)."] # [doc (hidden)] # [macro_export] macro_rules ! __handle_duplicate { ($ name : literal ;) => { } ; ($ name : literal ; $ ($ existing : tt) +) => { $ crate :: __macros :: compile_error ! ($ crate :: __macros :: concat ! ("cannot specify the `" , $ name , "` attribute twice" ,)) ; } ; }
};
}
