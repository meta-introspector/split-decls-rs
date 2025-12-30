// Generated macro for impl_792 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_792 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_792"}
// Dependencies: {}
impl UsesTypeParams for syn :: PathArguments { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { match * self { syn :: PathArguments :: None => Default :: default () , syn :: PathArguments :: AngleBracketed (ref v) => v . uses_type_params (options , type_set) , syn :: PathArguments :: Parenthesized (ref v) => v . uses_type_params (options , type_set) , } } }
};
}
