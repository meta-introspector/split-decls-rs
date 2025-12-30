// Generated macro for expr_to_string (function)
macro_rules! Depcrate_utilexpr_to_string {
() => {
// Module: crate::util
// Provides: {"expr_to_string"}
// Dependencies: {}
# [doc = " Extracts a string literal from the macro expanded version of `expr`,"] # [doc = " emitting `err_msg` if `expr` is not a string literal. This does not stop"] # [doc = " compilation on error, merely emits a non-fatal error and returns `Err`."] pub (crate) fn expr_to_string (cx : & mut ExtCtxt < '_ > , expr : Box < ast :: Expr > , err_msg : & 'static str ,) -> ExpandResult < Result < (Symbol , ast :: StrStyle) , ErrorGuaranteed > , () > { expr_to_spanned_string (cx , expr , err_msg) . map (| res | { res . map_err (| err | match err { Ok ((err , _)) => err . emit () , Err (guar) => guar , }) . map (| ExprToSpannedString { symbol , style , .. } | (symbol , style)) }) }
};
}
