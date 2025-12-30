// Generated macro for config_value (function)
macro_rules! Depcrate_attrsconfig_value {
() => {
// Module: crate::attrs
// Provides: {"config_value"}
// Dependencies: {}
# [doc = " Returns a string literal value if the given attribute is `value`"] # [doc = " attribute or `None` otherwise."] pub fn config_value (attr : & syn :: Attribute) -> Option < String > { get_name_value_str_lit (attr , "value") }
};
}
