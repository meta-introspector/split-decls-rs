// Generated macro for BuiltinShadowMode (enum)
macro_rules! Depcrate_item_scopeBuiltinShadowMode {
() => {
// Module: crate::item_scope
// Provides: {"BuiltinShadowMode"}
// Dependencies: {}
# [doc = " Shadow mode for builtin type which can be shadowed by module."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub (crate) enum BuiltinShadowMode { # [doc = " Prefer user-defined modules (or other types) over builtins."] Module , # [doc = " Prefer builtins over user-defined modules (but not other types)."] Other , }
};
}
