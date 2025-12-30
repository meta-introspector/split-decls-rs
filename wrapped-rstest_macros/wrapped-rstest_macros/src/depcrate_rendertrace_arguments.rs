// Generated macro for trace_arguments (function)
macro_rules! Depcrate_rendertrace_arguments {
() => {
// Module: crate::render
// Provides: {"trace_arguments"}
// Dependencies: {}
fn trace_arguments < 'a > (args : impl Iterator < Item = & 'a Pat > , attributes : & RsTestAttributes ,) -> Option < TokenStream > { let mut statements = args . filter (| & arg | attributes . trace_me (arg)) . map (| arg | { let s : Stmt = parse_quote ! { println ! ("{} = {:?}" , stringify ! (# arg) , # arg) ; } ; s }) . peekable () ; if statements . peek () . is_some () { Some (quote ! { println ! ("{:-^40}" , " TEST ARGUMENTS ") ; # (# statements) * println ! ("{:-^40}" , " TEST START ") ; }) } else { None } }
};
}
