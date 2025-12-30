// Generated macro for impl_795 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_795 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_795"}
// Dependencies: {}
impl UsesTypeParams for syn :: TypeParamBound { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { match * self { syn :: TypeParamBound :: Trait (ref v) => v . uses_type_params (options , type_set) , syn :: TypeParamBound :: Lifetime (_) => Default :: default () , _ => panic ! ("Unknown syn::TypeParamBound: {:?}" , self) , } } }
};
}
