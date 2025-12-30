// Generated macro for impl_239 (impl)
macro_rules! Depcrate_astimpl_239 {
() => {
// Module: crate::ast
// Provides: {"impl_239"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for TemplateParam where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_demangle_node (DemangleNodeType :: TemplateParam) ; let ret = if ctx . is_lambda_arg { write ! (ctx , "auto:{}" , self . 0 + 1) } else { let arg = self . resolve (scope) ? ; arg . demangle (ctx , scope) } ; ctx . pop_demangle_node () ; ret } }
};
}
