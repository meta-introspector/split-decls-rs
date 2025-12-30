// Generated macro for impl_758 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_758 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_758"}
// Dependencies: {}
impl < T : UsesTypeParams > UsesTypeParams for Option < T > { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { self . as_ref () . map (| v | v . uses_type_params (options , type_set)) . unwrap_or_default () } }
};
}
