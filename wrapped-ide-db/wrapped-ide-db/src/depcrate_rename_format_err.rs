// Generated macro for _format_err (macro)
macro_rules! Depcrate_rename_format_err {
() => {
// Module: crate::rename
// Provides: {"_format_err"}
// Dependencies: {}
# [macro_export] macro_rules ! _format_err { ($ fmt : expr) => { RenameError (format ! ($ fmt)) } ; ($ fmt : expr , $ ($ arg : tt) +) => { RenameError (format ! ($ fmt , $ ($ arg) +)) } }
};
}
