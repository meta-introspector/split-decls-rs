// Generated macro for impl_150 (impl)
macro_rules! Depcrate_astimpl_150 {
() => {
// Module: crate::ast
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for CloneTypeIdentifier where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; let ident = & ctx . input [self . start .. self . end] ; let source_name = String :: from_utf8_lossy (ident) ; ctx . set_source_name (self . start , self . end) ; write ! (ctx , " .{}" , source_name) ? ; Ok (()) } }
};
}
