// Generated macro for push_punct (macro)
macro_rules! Depcrate_tokens_runtimepush_punct {
() => {
// Module: crate::tokens::runtime
// Provides: {"push_punct"}
// Dependencies: {}
macro_rules ! push_punct { ($ name : ident $ char1 : tt) => { pub fn $ name (tokens : & mut TokenStream) { tokens . push_space () ; tokens . push ($ char1) ; } } ; ($ name : ident $ char1 : tt $ char2 : tt) => { pub fn $ name (tokens : & mut TokenStream) { tokens . push_space () ; tokens . push ($ char1) ; tokens . push ($ char2) ; } } ; ($ name : ident $ char1 : tt $ char2 : tt $ char3 : tt) => { pub fn $ name (tokens : & mut TokenStream) { tokens . push (' ') ; tokens . push ($ char1) ; tokens . push ($ char2) ; tokens . push ($ char3) ; } } ; }
};
}
