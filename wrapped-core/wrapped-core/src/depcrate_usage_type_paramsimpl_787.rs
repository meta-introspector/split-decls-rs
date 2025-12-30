// Generated macro for impl_787 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_787 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_787"}
// Dependencies: {}
# [doc = " Check if an Ident exactly matches one of the sought-after type parameters."] impl UsesTypeParams for Ident { fn uses_type_params < 'a > (& self , _options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { type_set . iter () . filter (| v | * v == self) . collect () } }
};
}
