// Generated macro for impl_84 (impl)
macro_rules! Depcrate_astimpl_84 {
() => {
// Module: crate::ast
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for MangledName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { MangledName :: Encoding (ref enc , ref cs) => { enc . demangle (ctx , scope) ? ; if ! cs . is_empty () && ctx . show_params { for clone_suffix in cs { clone_suffix . demangle (ctx , scope) ? ; } } Ok (()) } MangledName :: BlockInvoke (ref enc , _) => { write ! (ctx , "invocation function for block in ") ? ; enc . demangle (ctx , scope) ? ; Ok (()) } MangledName :: Type (ref ty) => ty . demangle (ctx , scope) , MangledName :: GlobalCtorDtor (ref gcd) => gcd . demangle (ctx , scope) , } } }
};
}
