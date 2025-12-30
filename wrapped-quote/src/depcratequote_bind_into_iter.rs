// Generated macro for quote_bind_into_iter (macro)
macro_rules! Depcratequote_bind_into_iter {
() => {
// Module: crate
// Provides: {"quote_bind_into_iter"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! quote_bind_into_iter { ($ has_iter : ident $ var : ident) => { # [allow (unused_mut)] let (mut $ var , i) = $ var . quote_into_iter () ; let $ has_iter = $ has_iter | i ; } ; }
};
}
