// Generated macro for impl_164 (impl)
macro_rules! Depcrate_check_proc_macroimpl_164 {
() => {
// Module: crate::check_proc_macro
// Provides: {"impl_164"}
// Dependencies: {}
impl < 'cx > WithSearchPat < 'cx > for (& FnKind < 'cx > , & Body < 'cx > , HirId , Span) { type Context = LateContext < 'cx > ; fn search_pat (& self , cx : & Self :: Context) -> (Pat , Pat) { fn_kind_pat (cx . tcx , self . 0 , self . 1 , self . 2) } fn span (& self) -> Span { self . 3 } }
};
}
