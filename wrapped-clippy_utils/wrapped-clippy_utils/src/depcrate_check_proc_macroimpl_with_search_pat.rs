// Generated macro for impl_with_search_pat (macro)
macro_rules! Depcrate_check_proc_macroimpl_with_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"impl_with_search_pat"}
// Dependencies: {}
macro_rules ! impl_with_search_pat { (($ cx_ident : ident : $ cx_ty : ident <$ cx_lt : lifetime >, $ self : tt : $ ty : ty) => $ fn : ident ($ ($ args : tt) *)) => { impl <$ cx_lt > WithSearchPat <$ cx_lt > for $ ty { type Context = $ cx_ty <$ cx_lt >; fn search_pat (&$ self , $ cx_ident : & Self :: Context) -> (Pat , Pat) { $ fn ($ ($ args) *) } fn span (& self) -> Span { self . span } } } ; }
};
}
