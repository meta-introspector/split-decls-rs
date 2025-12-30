// Generated macro for impl_109 (impl)
macro_rules! Depcrate_astimpl_109 {
() => {
// Module: crate::ast
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for UnscopedTemplateName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) } }
};
}
