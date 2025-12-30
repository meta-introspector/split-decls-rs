// Generated macro for impl_103 (impl)
macro_rules! Depcrate_astimpl_103 {
() => {
// Module: crate::ast
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for UnscopedName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { UnscopedName :: Unqualified (ref unqualified) => unqualified . demangle (ctx , scope) , UnscopedName :: Std (ref std) => { write ! (ctx , "std::") ? ; std . demangle (ctx , scope) } } } }
};
}
