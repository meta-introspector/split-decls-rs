// Generated macro for impl_97 (impl)
macro_rules! Depcrate_astimpl_97 {
() => {
// Module: crate::ast
// Provides: {"impl_97"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for Name where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { Name :: Nested (ref nested) => nested . demangle (ctx , scope) , Name :: Unscoped (ref unscoped) => unscoped . demangle (ctx , scope) , Name :: UnscopedTemplate (ref template , ref args) => { template . demangle (ctx , scope . push (args)) ? ; args . demangle (ctx , scope) } Name :: Local (ref local) => local . demangle (ctx , scope) , } } }
};
}
