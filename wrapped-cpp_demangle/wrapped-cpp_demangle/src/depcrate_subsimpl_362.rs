// Generated macro for impl_362 (impl)
macro_rules! Depcrate_subsimpl_362 {
() => {
// Module: crate::subs
// Provides: {"impl_362"}
// Dependencies: {}
impl ast :: IsCtorDtorConversion for Substitutable { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { match * self { Substitutable :: Prefix (ref prefix) => prefix . is_ctor_dtor_conversion (subs) , _ => false , } } }
};
}
