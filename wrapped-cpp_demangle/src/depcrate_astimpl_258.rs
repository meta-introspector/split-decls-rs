// Generated macro for impl_258 (impl)
macro_rules! Depcrate_astimpl_258 {
() => {
// Module: crate::ast
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for MemberName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; let needs_parens = self . 0 . get_template_args (ctx . subs) . is_some () ; if needs_parens { write ! (ctx , "(") ? ; } self . 0 . demangle (ctx , scope) ? ; if needs_parens { write ! (ctx , ")") ? ; } Ok (()) } }
};
}
