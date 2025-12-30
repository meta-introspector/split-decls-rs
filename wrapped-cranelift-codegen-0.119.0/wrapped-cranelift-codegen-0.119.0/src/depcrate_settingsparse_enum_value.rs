// Generated macro for parse_enum_value (function)
macro_rules! Depcrate_settingsparse_enum_value {
() => {
// Module: crate::settings
// Provides: {"parse_enum_value"}
// Dependencies: {}
fn parse_enum_value (value : & str , choices : & [& str]) -> SetResult < u8 > { match choices . iter () . position (| & tag | tag == value) { Some (idx) => Ok (idx as u8) , None => Err (SetError :: BadValue (format ! ("any among {}" , choices . join (", ")))) , } }
};
}
