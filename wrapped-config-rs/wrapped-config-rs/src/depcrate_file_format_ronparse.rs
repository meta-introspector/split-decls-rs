// Generated macro for parse (function)
macro_rules! Depcrate_file_format_ronparse {
() => {
// Module: crate::file::format::ron
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (uri : Option < & String > , text : & str ,) -> Result < Map < String , Value > , Box < dyn Error + Send + Sync > > { let value = from_ron_value (uri , ron :: from_str (text) ?) ? ; format :: extract_root_table (uri , value) }
};
}
