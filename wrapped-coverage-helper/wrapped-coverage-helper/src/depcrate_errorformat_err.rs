// Generated macro for format_err (macro)
macro_rules! Depcrate_errorformat_err {
() => {
// Module: crate::error
// Provides: {"format_err"}
// Dependencies: {}
macro_rules ! format_err { ($ span : expr , $ msg : expr $ (,) *) => { crate :: error :: Error :: new ($ span , String :: from ($ msg)) } ; ($ span : expr , $ ($ tt : tt) *) => { format_err ! ($ span , format ! ($ ($ tt) *)) } ; }
};
}
