// Generated macro for is_builtin_attr_name (function)
macro_rules! Depcrate_builtin_attrsis_builtin_attr_name {
() => {
// Module: crate::builtin_attrs
// Provides: {"is_builtin_attr_name"}
// Dependencies: {}
pub fn is_builtin_attr_name (name : Symbol) -> bool { BUILTIN_ATTRIBUTE_MAP . get (& name) . is_some () }
};
}
