// Generated macro for impl_162 (impl)
macro_rules! Depcrate_astimpl_162 {
() => {
// Module: crate::ast
// Provides: {"impl_162"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for CallOffset where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { CallOffset :: NonVirtual (NvOffset (offset)) => { write ! (ctx , "{{offset({})}}" , offset) ? ; } CallOffset :: Virtual (VOffset (vbase , vcall)) => { write ! (ctx , "{{virtual offset({}, {})}}" , vbase , vcall) ? ; } } Ok (()) } }
};
}
