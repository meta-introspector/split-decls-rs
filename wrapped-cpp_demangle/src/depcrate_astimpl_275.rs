// Generated macro for impl_275 (impl)
macro_rules! Depcrate_astimpl_275 {
() => {
// Module: crate::ast
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for SimpleId where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) ? ; if let Some (ref args) = self . 1 { args . demangle (ctx , scope) ? ; } Ok (()) } }
};
}
