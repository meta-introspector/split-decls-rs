// Generated macro for path_macros (macro)
macro_rules! Depcrate_pathspath_macros {
() => {
// Module: crate::paths
// Provides: {"path_macros"}
// Dependencies: {}
macro_rules ! path_macros { ($ ($ name : ident : $ ns : expr ,) *) => { $ (# [doc = " Only exported for tests and `clippy_lints_internal`"] # [doc (hidden)] # [macro_export] macro_rules ! $ name { ($$ ($$ seg : ident $$ (::) ?) *) => { PathLookup :: new ($ ns , & [$$ (sym ::$$ seg ,) *]) } ; }) * } ; }
};
}
