// Generated macro for impl_759 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_759 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_759"}
// Dependencies: {}
impl < T : UsesTypeParams > UsesTypeParams for Vec < T > { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { self . collect_type_params (options , type_set) } }
};
}
