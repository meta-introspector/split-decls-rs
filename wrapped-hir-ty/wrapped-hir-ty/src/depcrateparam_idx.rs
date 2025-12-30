// Generated macro for param_idx (function)
macro_rules! Depcrateparam_idx {
() => {
// Module: crate
// Provides: {"param_idx"}
// Dependencies: {}
# [doc = " Return an index of a parameter in the generic type parameter list by it's id."] pub fn param_idx (db : & dyn HirDatabase , id : TypeOrConstParamId) -> Option < usize > { generics :: generics (db , id . parent) . type_or_const_param_idx (id) }
};
}
