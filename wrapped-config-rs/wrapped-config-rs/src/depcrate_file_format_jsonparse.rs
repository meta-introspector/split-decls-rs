// Generated macro for parse (function)
macro_rules! Depcrate_file_format_jsonparse {
() => {
// Module: crate::file::format::json
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (uri : Option < & String > , text : & str ,) -> Result < Map < String , Value > , Box < dyn Error + Send + Sync > > { let value = from_json_value (uri , & serde_json :: from_str (text) ?) ; format :: extract_root_table (uri , value) }
};
}
