// Generated macro for parse_bool_value (function)
macro_rules! Depcrate_settingsparse_bool_value {
() => {
// Module: crate::settings
// Provides: {"parse_bool_value"}
// Dependencies: {}
fn parse_bool_value (value : & str) -> SetResult < bool > { match value { "true" | "on" | "yes" | "1" => Ok (true) , "false" | "off" | "no" | "0" => Ok (false) , _ => Err (SetError :: BadValue ("bool" . to_string ())) , } }
};
}
