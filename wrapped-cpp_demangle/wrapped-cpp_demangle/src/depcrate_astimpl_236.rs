// Generated macro for impl_236 (impl)
macro_rules! Depcrate_astimpl_236 {
() => {
// Module: crate::ast
// Provides: {"impl_236"}
// Dependencies: {}
impl < 'subs , W > DemangleAsInner < 'subs , W > for PointerToMemberType where W : 'subs + DemangleWrite , { fn demangle_as_inner < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle_as_inner ! (self , ctx , scope) ; if ctx . last_char_written != Some ('(') { ctx . ensure_space () ? ; } self . 0 . demangle (ctx , scope) ? ; write ! (ctx , "::*") ? ; Ok (()) } fn downcast_to_pointer_to_member (& self) -> Option < & PointerToMemberType > { Some (self) } }
};
}
