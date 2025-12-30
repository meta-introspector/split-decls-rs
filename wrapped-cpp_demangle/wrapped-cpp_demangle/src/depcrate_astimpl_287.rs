// Generated macro for impl_287 (impl)
macro_rules! Depcrate_astimpl_287 {
() => {
// Module: crate::ast
// Provides: {"impl_287"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for Initializer where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; write ! (ctx , "(") ? ; let mut need_comma = false ; for expr in & self . 0 { if need_comma { write ! (ctx , ", ") ? ; } expr . demangle (ctx , scope) ? ; need_comma = true ; } write ! (ctx , ")") ? ; Ok (()) } }
};
}
