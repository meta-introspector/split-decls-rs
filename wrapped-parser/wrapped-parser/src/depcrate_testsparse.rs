// Generated macro for parse (function)
macro_rules! Depcrate_testsparse {
() => {
// Module: crate::tests
// Provides: {"parse"}
// Dependencies: {}
fn parse (entry : TopEntryPoint , text : & str , edition : Edition) -> (String , bool) { let lexed = LexedStr :: new (edition , text) ; let input = lexed . to_input (edition) ; let output = entry . parse (& input , edition) ; let mut buf = String :: new () ; let mut errors = Vec :: new () ; let mut indent = String :: new () ; let mut depth = 0 ; let mut len = 0 ; lexed . intersperse_trivia (& output , & mut | step | match step { crate :: StrStep :: Token { kind , text } => { assert ! (depth > 0) ; len += text . len () ; writeln ! (buf , "{indent}{kind:?} {text:?}") . unwrap () ; } crate :: StrStep :: Enter { kind } => { assert ! (depth > 0 || len == 0) ; depth += 1 ; writeln ! (buf , "{indent}{kind:?}") . unwrap () ; indent . push_str ("  ") ; } crate :: StrStep :: Exit => { assert ! (depth > 0) ; depth -= 1 ; indent . pop () ; indent . pop () ; } crate :: StrStep :: Error { msg , pos } => { assert ! (depth > 0) ; errors . push (format ! ("error {pos}: {msg}\n")) } }) ; assert_eq ! (len , text . len () , "didn't parse all text.\nParsed:\n{}\n\nAll:\n{}\n" , & text [.. len] , text) ; for (token , msg) in lexed . errors () { let pos = lexed . text_start (token) ; errors . push (format ! ("error {pos}: {msg}\n")) ; } let has_errors = ! errors . is_empty () ; for e in errors { buf . push_str (& e) ; } (buf , has_errors) }
};
}
