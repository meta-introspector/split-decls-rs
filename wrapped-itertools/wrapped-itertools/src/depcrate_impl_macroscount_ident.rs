// Generated macro for count_ident (macro)
macro_rules! Depcrate_impl_macroscount_ident {
() => {
// Module: crate::impl_macros
// Provides: {"count_ident"}
// Dependencies: {}
macro_rules ! count_ident { () => { 0 } ; ($ i0 : ident $ ($ i : ident) *) => { 1 + count_ident ! ($ ($ i) *) } ; }
};
}
