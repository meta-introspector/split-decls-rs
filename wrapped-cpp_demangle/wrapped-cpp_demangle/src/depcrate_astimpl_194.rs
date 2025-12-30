// Generated macro for impl_194 (impl)
macro_rules! Depcrate_astimpl_194 {
() => {
// Module: crate::ast
// Provides: {"impl_194"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for BuiltinType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { BuiltinType :: Standard (ref ty) => ty . demangle (ctx , scope) , BuiltinType :: Parametric (ref ty) => ty . demangle (ctx , scope) , BuiltinType :: Extension (ref name) => name . demangle (ctx , scope) , } } }
};
}
