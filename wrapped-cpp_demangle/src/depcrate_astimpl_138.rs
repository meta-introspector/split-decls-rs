// Generated macro for impl_138 (impl)
macro_rules! Depcrate_astimpl_138 {
() => {
// Module: crate::ast
// Provides: {"impl_138"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for SourceName where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) } }
};
}
