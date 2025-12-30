// Generated macro for impl_184 (impl)
macro_rules! Depcrate_astimpl_184 {
() => {
// Module: crate::ast
// Provides: {"impl_184"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for CvQualifiers where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; if self . const_ { ctx . ensure_space () ? ; write ! (ctx , "const") ? ; } if self . volatile { ctx . ensure_space () ? ; write ! (ctx , "volatile") ? ; } if self . restrict { ctx . ensure_space () ? ; write ! (ctx , "restrict") ? ; } Ok (()) } }
};
}
