// Generated macro for quote_append (macro)
macro_rules! Depcrate_astquote_append {
() => {
// Module: crate::ast
// Provides: {"quote_append"}
// Dependencies: {}
macro_rules ! quote_append { ($ tokens : expr , $ ($ quasi : tt) *) => { $ tokens . append_all (quote ! ($ ($ quasi) *)) } ; }
};
}
