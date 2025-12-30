// Generated macro for parse (function)
macro_rules! Depcrate_file_format_tomlparse {
() => {
// Module: crate::file::format::toml
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (uri : Option < & String > , text : & str ,) -> Result < Map < String , Value > , Box < dyn Error + Send + Sync > > { let table = from_toml_table (uri , toml :: from_str (text) ?) ; Ok (table) }
};
}
