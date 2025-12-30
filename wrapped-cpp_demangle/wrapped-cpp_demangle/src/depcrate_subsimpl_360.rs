// Generated macro for impl_360 (impl)
macro_rules! Depcrate_subsimpl_360 {
() => {
// Module: crate::subs
// Provides: {"impl_360"}
// Dependencies: {}
impl < 'subs , W > ast :: Demangle < 'subs , W > for Substitutable where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut ast :: DemangleContext < 'subs , W > , scope : Option < ast :: ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { match * self { Substitutable :: UnscopedTemplateName (ref name) => name . demangle (ctx , scope) , Substitutable :: Type (ref ty) => ty . demangle (ctx , scope) , Substitutable :: TemplateTemplateParam (ref ttp) => ttp . demangle (ctx , scope) , Substitutable :: UnresolvedType (ref ty) => ty . demangle (ctx , scope) , Substitutable :: Prefix (ref prefix) => prefix . demangle (ctx , scope) , } } }
};
}
