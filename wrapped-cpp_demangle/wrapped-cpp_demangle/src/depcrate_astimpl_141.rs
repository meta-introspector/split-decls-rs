// Generated macro for impl_141 (impl)
macro_rules! Depcrate_astimpl_141 {
() => {
// Module: crate::ast
// Provides: {"impl_141"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for AbiTags where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; for tag in & self . 0 { tag . demangle (ctx , scope) ? ; } Ok (()) } }
};
}
