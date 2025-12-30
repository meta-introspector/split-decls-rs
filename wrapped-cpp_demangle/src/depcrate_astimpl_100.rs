// Generated macro for impl_100 (impl)
macro_rules! Depcrate_astimpl_100 {
() => {
// Module: crate::ast
// Provides: {"impl_100"}
// Dependencies: {}
impl IsCtorDtorConversion for Name { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { match * self { Name :: Unscoped (ref unscoped) => unscoped . is_ctor_dtor_conversion (subs) , Name :: Nested (ref nested) => nested . is_ctor_dtor_conversion (subs) , Name :: Local (_) | Name :: UnscopedTemplate (..) => false , } } }
};
}
