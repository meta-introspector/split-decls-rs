// Generated macro for impl_201 (impl)
macro_rules! Depcrate_astimpl_201 {
() => {
// Module: crate::ast
// Provides: {"impl_201"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for ExceptionSpec where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { ExceptionSpec :: NoExcept => write ! (ctx , "noexcept") , ExceptionSpec :: Computed (ref expr) => { write ! (ctx , "noexcept(") ? ; expr . demangle (ctx , scope) ? ; write ! (ctx , ")") } } } }
};
}
