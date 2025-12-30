// Generated macro for impl_144 (impl)
macro_rules! Depcrate_astimpl_144 {
() => {
// Module: crate::ast
// Provides: {"impl_144"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for AbiTag where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; write ! (ctx , "[abi:") ? ; self . 0 . demangle (ctx , scope) ? ; write ! (ctx , "]") } }
};
}
