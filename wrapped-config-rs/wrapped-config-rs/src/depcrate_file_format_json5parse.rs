// Generated macro for parse (function)
macro_rules! Depcrate_file_format_json5parse {
() => {
// Module: crate::file::format::json5
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (uri : Option < & String > , text : & str ,) -> Result < Map < String , Value > , Box < dyn Error + Send + Sync > > { let value = from_json5_value (uri , json5_rs :: from_str :: < Val > (text) ?) ; format :: extract_root_table (uri , value) }
};
}
