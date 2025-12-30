// Generated macro for from_toml_value (function)
macro_rules! Depcrate_file_format_tomlfrom_toml_value {
() => {
// Module: crate::file::format::toml
// Provides: {"from_toml_value"}
// Dependencies: {}
fn from_toml_value (uri : Option < & String > , value : toml :: Value) -> Value { match value { toml :: Value :: String (value) => Value :: new (uri , value) , toml :: Value :: Float (value) => Value :: new (uri , value) , toml :: Value :: Integer (value) => Value :: new (uri , value) , toml :: Value :: Boolean (value) => Value :: new (uri , value) , toml :: Value :: Table (table) => { let m = from_toml_table (uri , table) ; Value :: new (uri , m) } toml :: Value :: Array (array) => { let mut l = Vec :: new () ; for value in array { l . push (from_toml_value (uri , value)) ; } Value :: new (uri , l) } toml :: Value :: Datetime (datetime) => Value :: new (uri , datetime . to_string ()) , } }
};
}
