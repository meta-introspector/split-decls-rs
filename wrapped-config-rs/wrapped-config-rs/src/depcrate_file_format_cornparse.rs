// Generated macro for parse (function)
macro_rules! Depcrate_file_format_cornparse {
() => {
// Module: crate::file::format::corn
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (uri : Option < & String > , text : & str ,) -> Result < Map < String , Value > , Box < dyn Error + Send + Sync > > { let value = from_corn_value (uri , & corn :: parse (text) ?) ; format :: extract_root_table (uri , value) }
};
}
