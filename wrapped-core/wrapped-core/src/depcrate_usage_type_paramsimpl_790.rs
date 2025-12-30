// Generated macro for impl_790 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_790 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_790"}
// Dependencies: {}
impl UsesTypeParams for syn :: TypePath { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { let hits = self . path . uses_type_params (options , type_set) ; if options . include_type_path_qself () { union_in_place (hits , self . qself . uses_type_params (options , type_set)) } else { hits } } }
};
}
