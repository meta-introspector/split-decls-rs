// Generated macro for impl_197 (impl)
macro_rules! Depcrate_astimpl_197 {
() => {
// Module: crate::ast
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for QualifiedBuiltin where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_inner (& self . 0) ; self . 1 . demangle (ctx , scope) ? ; if ctx . pop_inner_if (& self . 0) { self . 0 . demangle_as_inner (ctx , scope) ? ; } Ok (()) } }
};
}
