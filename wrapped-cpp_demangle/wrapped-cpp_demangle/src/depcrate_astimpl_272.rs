// Generated macro for impl_272 (impl)
macro_rules! Depcrate_astimpl_272 {
() => {
// Module: crate::ast
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for UnresolvedQualifierLevel where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) } }
};
}
