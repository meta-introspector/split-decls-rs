// Generated macro for impl_290 (impl)
macro_rules! Depcrate_astimpl_290 {
() => {
// Module: crate::ast
// Provides: {"impl_290"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for LocalName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; let saved_show_params = ctx . show_params ; ctx . show_params = true ; let ret = match * self { LocalName :: Relative (ref encoding , Some (ref name) , _) => { encoding . demangle (ctx , scope) ? ; write ! (ctx , "::") ? ; name . demangle (ctx , scope) } LocalName :: Relative (ref encoding , None , _) => { encoding . demangle (ctx , scope) ? ; write ! (ctx , "::string literal") ? ; Ok (()) } LocalName :: Default (ref encoding , _ , _) => encoding . demangle (ctx , scope) , } ; ctx . show_params = saved_show_params ; ret } }
};
}
