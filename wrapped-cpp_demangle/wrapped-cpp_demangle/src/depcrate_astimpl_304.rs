// Generated macro for impl_304 (impl)
macro_rules! Depcrate_astimpl_304 {
() => {
// Module: crate::ast
// Provides: {"impl_304"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for LambdaSig where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . is_lambda_arg = true ; let r = self . demangle_args (ctx , scope) ; ctx . is_lambda_arg = false ; r } }
};
}
