// Generated macro for trace_rustc (function)
macro_rules! Depcratetrace_rustc {
() => {
// Module: crate
// Provides: {"trace_rustc"}
// Dependencies: {}
# [decl2 (fn , name = "extract" , vis = "pub" , hash = "a9586077")] # [proc_macro] # [decl2 (fn , name = "trace_rustc" , vis = "pub" , hash = "b5e8dbb3")] pub fn trace_rustc (input : TokenStream) -> TokenStream { rustc_tracer :: trace_rustc_impl (input) }
};
}
