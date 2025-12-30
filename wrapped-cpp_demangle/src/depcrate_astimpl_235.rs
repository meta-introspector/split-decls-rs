// Generated macro for impl_235 (impl)
macro_rules! Depcrate_astimpl_235 {
() => {
// Module: crate::ast
// Provides: {"impl_235"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for PointerToMemberType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_inner (self) ; self . 1 . demangle (ctx , scope) ? ; if ctx . pop_inner_if (self) { self . demangle_as_inner (ctx , scope) ? ; } Ok (()) } }
};
}
