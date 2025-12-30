// Generated macro for impl_760 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_760 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_760"}
// Dependencies: {}
impl < T : UsesTypeParams , U > UsesTypeParams for Punctuated < T , U > { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { self . collect_type_params (options , type_set) } }
};
}
