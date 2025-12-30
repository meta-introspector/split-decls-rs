// Generated macro for to_json (function)
macro_rules! Depcrate_json_valueto_json {
() => {
// Module: crate::json::value
// Provides: {"to_json"}
// Dependencies: {}
# [doc = " Convert any serializable data into Serde Json type"] pub fn to_json < T > (src : T) -> Json where T : Serialize , { to_value (src) . unwrap_or_default () }
};
}
