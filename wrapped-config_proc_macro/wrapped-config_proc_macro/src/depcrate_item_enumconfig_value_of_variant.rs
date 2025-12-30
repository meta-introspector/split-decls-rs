// Generated macro for config_value_of_variant (function)
macro_rules! Depcrate_item_enumconfig_value_of_variant {
() => {
// Module: crate::item_enum
// Provides: {"config_value_of_variant"}
// Dependencies: {}
fn config_value_of_variant (variant : & syn :: Variant) -> String { find_config_value (& variant . attrs) . unwrap_or (variant . ident . to_string ()) }
};
}
