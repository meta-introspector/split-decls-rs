// Generated macro for _error (macro)
macro_rules! Depcrate_errors_error {
() => {
// Module: crate::errors
// Provides: {"_error"}
// Dependencies: {}
# [doc = " Constructs an SsrError taking arguments like the format macro."] macro_rules ! _error { ($ fmt : expr) => { $ crate :: SsrError :: new (format ! ($ fmt)) } ; ($ fmt : expr , $ ($ arg : tt) +) => { $ crate :: SsrError :: new (format ! ($ fmt , $ ($ arg) +)) } }
};
}
