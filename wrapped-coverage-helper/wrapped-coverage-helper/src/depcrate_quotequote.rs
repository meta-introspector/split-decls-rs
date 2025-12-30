// Generated macro for quote (macro)
macro_rules! Depcrate_quotequote {
() => {
// Module: crate::quote
// Provides: {"quote"}
// Dependencies: {}
macro_rules ! quote { ($ ($ tt : tt) *) => { { let mut tokens = :: proc_macro :: TokenStream :: new () ; quote_each_token ! (tokens $ ($ tt) *) ; tokens } } ; }
};
}
