// Generated macro for DelimSpanEnum (enum)
macro_rules! Depcrate_extraDelimSpanEnum {
() => {
// Module: crate::extra
// Provides: {"DelimSpanEnum"}
// Dependencies: {}
# [derive (Copy , Clone)] enum DelimSpanEnum { # [cfg (wrap_proc_macro)] Compiler { join : proc_macro :: Span , open : proc_macro :: Span , close : proc_macro :: Span , } , Fallback (fallback :: Span) , }
};
}
