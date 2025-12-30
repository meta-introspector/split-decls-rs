// Generated macro for impl_245 (impl)
macro_rules! Depcrate_astimpl_245 {
() => {
// Module: crate::ast
// Provides: {"impl_245"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for TemplateTemplateParam where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) } }
};
}
