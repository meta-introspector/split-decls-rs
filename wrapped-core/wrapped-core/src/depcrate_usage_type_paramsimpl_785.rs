// Generated macro for impl_785 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_785 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_785"}
// Dependencies: {}
impl UsesTypeParams for syn :: Data { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { match * self { syn :: Data :: Struct (ref v) => v . uses_type_params (options , type_set) , syn :: Data :: Enum (ref v) => v . uses_type_params (options , type_set) , syn :: Data :: Union (ref v) => v . uses_type_params (options , type_set) , } } }
};
}
