// Generated macro for try_stream_inner (function)
macro_rules! Depcratetry_stream_inner {
() => {
// Module: crate
// Provides: {"try_stream_inner"}
// Dependencies: {}
# [doc = " The first token tree in the stream must be a group containing the path to the `async-stream`"] # [doc = " crate."] # [proc_macro] # [doc (hidden)] pub fn try_stream_inner (input : TokenStream) -> TokenStream { let (crate_path , mut stmts) = match parse_input (input) { Ok (x) => x , Err (e) => return e . to_compile_error () . into () , } ; let mut scrub = Scrub :: new (true , & crate_path) ; for stmt in & mut stmts { scrub . visit_stmt_mut (stmt) ; } let dummy_yield = if scrub . has_yielded { None } else { Some (quote ! (if false { __yield_tx . send (()) . await ; })) } ; quote ! ({ let (mut __yield_tx , __yield_rx) = unsafe { # crate_path :: __private :: yielder :: pair () } ; # crate_path :: __private :: AsyncStream :: new (__yield_rx , async move { '__async_stream_private_check_scope : { # dummy_yield # (# stmts) * } }) }) . into () }
};
}
