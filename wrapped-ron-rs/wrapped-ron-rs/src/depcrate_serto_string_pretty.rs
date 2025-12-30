// Generated macro for to_string_pretty (function)
macro_rules! Depcrate_serto_string_pretty {
() => {
// Module: crate::ser
// Provides: {"to_string_pretty"}
// Dependencies: {}
# [doc = " Serializes `value` in the recommended RON layout in a pretty way."] pub fn to_string_pretty < T > (value : & T , config : PrettyConfig) -> Result < String > where T : ? Sized + Serialize , { Options :: default () . to_string_pretty (value , config) }
};
}
