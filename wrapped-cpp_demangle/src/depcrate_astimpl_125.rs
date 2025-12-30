// Generated macro for impl_125 (impl)
macro_rules! Depcrate_astimpl_125 {
() => {
// Module: crate::ast
// Provides: {"impl_125"}
// Dependencies: {}
impl IsCtorDtorConversion for Prefix { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { match * self { Prefix :: Unqualified (ref unqualified) | Prefix :: Nested (_ , ref unqualified) => { unqualified . is_ctor_dtor_conversion (subs) } Prefix :: Template (ref prefix , _) => prefix . is_ctor_dtor_conversion (subs) , _ => false , } } }
};
}
