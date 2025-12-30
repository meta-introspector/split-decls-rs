// Generated macro for format_err (macro)
macro_rules! Depcrate_errorformat_err {
() => {
// Module: crate::error
// Provides: {"format_err"}
// Dependencies: {}
macro_rules ! format_err { ($ span : expr , $ msg : expr $ (,) ?) => { syn :: Error :: new_spanned (&$ span as & dyn quote :: ToTokens , &$ msg as & dyn std :: fmt :: Display) } ; ($ span : expr , $ ($ tt : tt) *) => { format_err ! ($ span , format ! ($ ($ tt) *)) } ; }
};
}
