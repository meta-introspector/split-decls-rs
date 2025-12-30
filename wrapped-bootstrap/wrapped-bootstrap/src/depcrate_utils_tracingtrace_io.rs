// Generated macro for trace_io (macro)
macro_rules! Depcrate_utils_tracingtrace_io {
() => {
// Module: crate::utils::tracing
// Provides: {"trace_io"}
// Dependencies: {}
# [doc = " Create a tracing span around an I/O operation, if tracing is enabled."] # [doc = " Note that at least one tracing value field has to be passed to this macro, otherwise it will not"] # [doc = " compile."] # [macro_export] macro_rules ! trace_io { ($ name : expr , $ ($ args : tt) *) => { :: tracing :: trace_span ! (target : $ crate :: utils :: tracing :: IO_SPAN_TARGET , $ name , $ ($ args) *, location = $ crate :: utils :: tracing :: format_location (*:: std :: panic :: Location :: caller ())) . entered () } }
};
}
