// Generated macro for impl_75 (impl)
macro_rules! Depcrate_astimpl_75 {
() => {
// Module: crate::ast
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for FunctionArgListAndReturnType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { FunctionArgSlice :: new (& self . 0 [1 ..]) . demangle (ctx , scope) } }
};
}
