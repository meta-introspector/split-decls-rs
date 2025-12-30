// Generated macro for impl_788 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_788 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_788"}
// Dependencies: {}
impl UsesTypeParams for syn :: ReturnType { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { if let syn :: ReturnType :: Type (_ , ref ty) = * self { ty . uses_type_params (options , type_set) } else { Default :: default () } } }
};
}
