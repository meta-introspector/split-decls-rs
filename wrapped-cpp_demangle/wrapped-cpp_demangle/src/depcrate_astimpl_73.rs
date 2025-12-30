// Generated macro for impl_73 (impl)
macro_rules! Depcrate_astimpl_73 {
() => {
// Module: crate::ast
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for FunctionArgList where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { FunctionArgSlice :: new (& self . 0 [..]) . demangle (ctx , scope) } }
};
}
