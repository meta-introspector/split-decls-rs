// Generated macro for impl_117 (impl)
macro_rules! Depcrate_astimpl_117 {
() => {
// Module: crate::ast
// Provides: {"impl_117"}
// Dependencies: {}
impl IsCtorDtorConversion for NestedName { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { self . prefix () . map (| p | p . is_ctor_dtor_conversion (subs)) . unwrap_or (false) } }
};
}
