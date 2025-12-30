// Generated macro for impl_126 (impl)
macro_rules! Depcrate_astimpl_126 {
() => {
// Module: crate::ast
// Provides: {"impl_126"}
// Dependencies: {}
impl IsCtorDtorConversion for PrefixHandle { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { match * self { PrefixHandle :: BackReference (idx) => { if let Some (sub) = subs . get (idx) { sub . is_ctor_dtor_conversion (subs) } else { false } } PrefixHandle :: NonSubstitution (NonSubstitution (idx)) => { if let Some (sub) = subs . get_non_substitution (idx) { sub . is_ctor_dtor_conversion (subs) } else { false } } PrefixHandle :: WellKnown (_) => false , } } }
};
}
