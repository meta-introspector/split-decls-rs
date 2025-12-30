// Generated macro for impl_105 (impl)
macro_rules! Depcrate_astimpl_105 {
() => {
// Module: crate::ast
// Provides: {"impl_105"}
// Dependencies: {}
impl IsCtorDtorConversion for UnscopedName { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { match * self { UnscopedName :: Unqualified (ref name) | UnscopedName :: Std (ref name) => { name . is_ctor_dtor_conversion (subs) } } } }
};
}
