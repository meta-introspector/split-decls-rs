// Generated macro for impl_789 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_789 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_789"}
// Dependencies: {}
impl UsesTypeParams for Type { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { match * self { Type :: Slice (ref v) => v . uses_type_params (options , type_set) , Type :: Array (ref v) => v . uses_type_params (options , type_set) , Type :: Ptr (ref v) => v . uses_type_params (options , type_set) , Type :: Reference (ref v) => v . uses_type_params (options , type_set) , Type :: BareFn (ref v) => v . uses_type_params (options , type_set) , Type :: Tuple (ref v) => v . uses_type_params (options , type_set) , Type :: Path (ref v) => v . uses_type_params (options , type_set) , Type :: Paren (ref v) => v . uses_type_params (options , type_set) , Type :: Group (ref v) => v . uses_type_params (options , type_set) , Type :: TraitObject (ref v) => v . uses_type_params (options , type_set) , Type :: ImplTrait (ref v) => v . uses_type_params (options , type_set) , Type :: Macro (_) | Type :: Verbatim (_) | Type :: Infer (_) | Type :: Never (_) => { Default :: default () } _ => panic ! ("Unknown syn::Type: {:?}" , self) , } } }
};
}
