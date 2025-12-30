// Generated macro for impl_308 (impl)
macro_rules! Depcrate_astimpl_308 {
() => {
// Module: crate::ast
// Provides: {"impl_308"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for DataMemberPrefix where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_demangle_node (DemangleNodeType :: DataMemberPrefix) ; let ret = self . 0 . demangle (ctx , scope) ; ctx . pop_demangle_node () ; ret } }
};
}
