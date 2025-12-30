// Generated macro for impl_794 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_794 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_794"}
// Dependencies: {}
impl UsesTypeParams for syn :: GenericArgument { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { match * self { syn :: GenericArgument :: Type (ref v) => v . uses_type_params (options , type_set) , syn :: GenericArgument :: AssocType (ref v) => v . uses_type_params (options , type_set) , syn :: GenericArgument :: Constraint (ref v) => v . uses_type_params (options , type_set) , syn :: GenericArgument :: AssocConst (_) | syn :: GenericArgument :: Const (_) | syn :: GenericArgument :: Lifetime (_) => Default :: default () , _ => panic ! ("Unknown syn::GenericArgument: {:?}" , self) , } } }
};
}
