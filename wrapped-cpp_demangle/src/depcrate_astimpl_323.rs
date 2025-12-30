// Generated macro for impl_323 (impl)
macro_rules! Depcrate_astimpl_323 {
() => {
// Module: crate::ast
// Provides: {"impl_323"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for SubobjectExpr where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . expr . demangle (ctx , scope) ? ; write ! (ctx , ".<") ? ; self . ty . demangle (ctx , scope) ? ; write ! (ctx , " at offset {}>" , self . offset) } }
};
}
