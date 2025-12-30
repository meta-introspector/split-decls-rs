// Generated macro for impl_214 (impl)
macro_rules! Depcrate_astimpl_214 {
() => {
// Module: crate::ast
// Provides: {"impl_214"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for Decltype where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_demangle_node (DemangleNodeType :: TemplateParam) ; let ret = match * self { Decltype :: Expression (ref expr) | Decltype :: IdExpression (ref expr) => { write ! (ctx , "decltype (") ? ; expr . demangle (ctx , scope) ? ; write ! (ctx , ")") ? ; Ok (()) } } ; ctx . pop_demangle_node () ; ret } }
};
}
