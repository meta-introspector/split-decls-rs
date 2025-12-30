// Generated macro for from_toml_table (function)
macro_rules! Depcrate_file_format_tomlfrom_toml_table {
() => {
// Module: crate::file::format::toml
// Provides: {"from_toml_table"}
// Dependencies: {}
fn from_toml_table (uri : Option < & String > , table : toml :: Table) -> Map < String , Value > { let mut m = Map :: new () ; for (key , value) in table { m . insert (key , from_toml_value (uri , value)) ; } m }
};
}
