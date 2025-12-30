// Generated macro for helper (macro)
macro_rules! Depcrate_implshelper {
() => {
// Module: crate::impls
// Provides: {"helper"}
// Dependencies: {}
# [doc = " Helper macro to call a `callback` macro four times for all combinations of"] # [doc = " `proc_macro`/`proc_macro2` and `&`/owned."] macro_rules ! helper { ($ callback : ident , $ ($ input : tt) *) => { $ callback ! ([proc_macro ::] => $ ($ input) *) ; $ callback ! ([& proc_macro ::] => $ ($ input) *) ; # [cfg (feature = "proc-macro2")] $ callback ! ([proc_macro2 ::] => $ ($ input) *) ; # [cfg (feature = "proc-macro2")] $ callback ! ([& proc_macro2 ::] => $ ($ input) *) ; } ; }
};
}
