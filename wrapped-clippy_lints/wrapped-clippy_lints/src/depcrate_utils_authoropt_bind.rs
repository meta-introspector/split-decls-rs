// Generated macro for opt_bind (macro)
macro_rules! Depcrate_utils_authoropt_bind {
() => {
// Module: crate::utils::author
// Provides: {"opt_bind"}
// Dependencies: {}
# [doc = " Transforms the given `Option<T>` variables into `OptionPat<Binding<T>>`."] # [doc = " This displays as `Some($name)` or `None` when printed. The name of the inner binding"] # [doc = " is set to the name of the variable passed to the macro."] macro_rules ! opt_bind { ($ self : ident $ (, $ name : ident) +) => { $ (let $ name = OptionPat :: new ($ name . map (| o | $ self . bind (stringify ! ($ name) , o))) ;) + } ; }
};
}
