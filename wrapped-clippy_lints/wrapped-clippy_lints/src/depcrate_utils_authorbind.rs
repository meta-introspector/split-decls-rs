// Generated macro for bind (macro)
macro_rules! Depcrate_utils_authorbind {
() => {
// Module: crate::utils::author
// Provides: {"bind"}
// Dependencies: {}
# [doc = " The variables passed in are replaced with `&Binding`s where the `value` field is set"] # [doc = " to the original value of the variable. The `name` field is set to the name of the variable"] # [doc = " (using `stringify!`) and is adjusted to avoid duplicate names."] # [doc = " Note that the `Binding` may be printed directly to output the `name`."] macro_rules ! bind { ($ self : ident $ (, $ name : ident) +) => { $ (let $ name = & $ self . bind (stringify ! ($ name) , $ name) ;) + } ; }
};
}
