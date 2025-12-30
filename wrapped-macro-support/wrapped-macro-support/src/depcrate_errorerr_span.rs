// Generated macro for err_span (macro)
macro_rules! Depcrate_errorerr_span {
() => {
// Module: crate::error
// Provides: {"err_span"}
// Dependencies: {}
# [doc = " Provide a Diagnostic with the given span and message"] macro_rules ! err_span { ($ span : expr , $ ($ msg : tt) *) => ($ crate :: Diagnostic :: spanned_error (&$ span , format ! ($ ($ msg) *))) }
};
}
