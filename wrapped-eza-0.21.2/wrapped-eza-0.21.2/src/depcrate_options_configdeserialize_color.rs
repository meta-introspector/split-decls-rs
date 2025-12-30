// Generated macro for deserialize_color (function)
macro_rules! Depcrate_options_configdeserialize_color {
() => {
// Module: crate::options::config
// Provides: {"deserialize_color"}
// Dependencies: {}
# [rustfmt :: skip] fn deserialize_color < 'de , D > (deserializer : D) -> Result < Option < Color > , D :: Error > where D : Deserializer < 'de > { Ok (color_from_str (& String :: deserialize (deserializer) ?)) }
};
}
