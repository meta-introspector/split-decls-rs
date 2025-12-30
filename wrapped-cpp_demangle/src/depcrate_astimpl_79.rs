// Generated macro for impl_79 (impl)
macro_rules! Depcrate_astimpl_79 {
() => {
// Module: crate::ast
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for NonSubstitution where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { ctx . subs . non_substitution (self . 0) . demangle (ctx , scope) } }
};
}
