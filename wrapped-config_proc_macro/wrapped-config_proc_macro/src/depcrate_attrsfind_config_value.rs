// Generated macro for find_config_value (function)
macro_rules! Depcrate_attrsfind_config_value {
() => {
// Module: crate::attrs
// Provides: {"find_config_value"}
// Dependencies: {}
# [doc = " Returns the value of the first `value` attribute in the given slice or"] # [doc = " `None` if `value` attribute is not available."] pub fn find_config_value (attrs : & [syn :: Attribute]) -> Option < String > { attrs . iter () . filter_map (config_value) . next () }
};
}
