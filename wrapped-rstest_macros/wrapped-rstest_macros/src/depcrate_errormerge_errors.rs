// Generated macro for merge_errors (macro)
macro_rules! Depcrate_errormerge_errors {
() => {
// Module: crate::error
// Provides: {"merge_errors"}
// Dependencies: {}
macro_rules ! merge_errors { ($ e : expr) => { $ e } ; ($ e : expr , $ ($ es : expr) , + $ (,) ?) => { crate :: error :: _merge_errors ($ e , merge_errors ! ($ ($ es) ,*)) } ; }
};
}
