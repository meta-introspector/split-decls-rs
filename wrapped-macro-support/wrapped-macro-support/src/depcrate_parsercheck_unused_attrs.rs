// Generated macro for check_unused_attrs (function)
macro_rules! Depcrate_parsercheck_unused_attrs {
() => {
// Module: crate::parser
// Provides: {"check_unused_attrs"}
// Dependencies: {}
pub fn check_unused_attrs (tokens : & mut TokenStream) { ATTRS . with (| state | { assert_eq ! (state . parsed . get () , state . checks . get ()) ; let unused_attrs = & * state . unused_attrs . borrow () ; if ! unused_attrs . is_empty () { let unused_attrs = unused_attrs . iter () . map (| UnusedState { error , ident } | { if * error { let text = format ! ("invalid attribute {} in this position" , ident) ; quote :: quote_spanned ! { ident . span () => :: core :: compile_error ! (# text) ; } } else { quote :: quote ! { let # ident : () ; } } }) ; tokens . extend (quote :: quote ! { const _ : () = { # (# unused_attrs) * } ; }) ; } }) }
};
}
