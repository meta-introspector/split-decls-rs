// Generated macro for bail (macro)
macro_rules! Depcrate_syn_utilsbail {
() => {
// Module: crate::syn_utils
// Provides: {"bail"}
// Dependencies: {}
macro_rules ! bail { (_ , $ ($ arg : tt) *) => { bail ! (proc_macro2 :: Span :: call_site () , $ ($ arg) *) } ; ($ span : expr , $ fmt : expr $ (,) ?) => { return std :: result :: Result :: Err (syn :: Error :: new ($ span , std :: format ! ($ fmt))) } ; ($ span : expr , $ fmt : expr , $ ($ arg : tt) *) => { return std :: result :: Result :: Err (syn :: Error :: new ($ span , std :: format ! ($ fmt , $ ($ arg) *))) } ; }
};
}
