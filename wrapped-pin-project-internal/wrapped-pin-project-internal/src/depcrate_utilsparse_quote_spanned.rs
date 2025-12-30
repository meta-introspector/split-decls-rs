// Generated macro for parse_quote_spanned (macro)
macro_rules! Depcrate_utilsparse_quote_spanned {
() => {
// Module: crate::utils
// Provides: {"parse_quote_spanned"}
// Dependencies: {}
macro_rules ! parse_quote_spanned { ($ span : expr => $ ($ tt : tt) *) => { syn :: parse2 (quote :: quote_spanned ! ($ span => $ ($ tt) *)) . unwrap_or_else (| e | panic ! ("{}" , e)) } ; }
};
}
