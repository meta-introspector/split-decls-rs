// Generated macro for helper_no_refs (macro)
macro_rules! Depcrate_implshelper_no_refs {
() => {
// Module: crate::impls
// Provides: {"helper_no_refs"}
// Dependencies: {}
# [doc = " Like `helper!` but without reference types."] macro_rules ! helper_no_refs { ($ callback : ident , $ ($ input : tt) *) => { $ callback ! ([proc_macro ::] => $ ($ input) *) ; # [cfg (feature = "proc-macro2")] $ callback ! ([proc_macro2 ::] => $ ($ input) *) ; } ; }
};
}
