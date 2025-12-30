// Generated macro for impl_793 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_793 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_793"}
// Dependencies: {}
impl UsesTypeParams for syn :: WherePredicate { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { match * self { syn :: WherePredicate :: Lifetime (_) => Default :: default () , syn :: WherePredicate :: Type (ref v) => v . uses_type_params (options , type_set) , _ => panic ! ("Unknown syn::WherePredicate: {:?}" , self) , } } }
};
}
