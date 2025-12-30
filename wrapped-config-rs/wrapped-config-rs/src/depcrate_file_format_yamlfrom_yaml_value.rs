// Generated macro for from_yaml_value (function)
macro_rules! Depcrate_file_format_yamlfrom_yaml_value {
() => {
// Module: crate::file::format::yaml
// Provides: {"from_yaml_value"}
// Dependencies: {}
fn from_yaml_value (uri : Option < & String > , value : & yaml :: Yaml ,) -> Result < Value , Box < dyn Error + Send + Sync > > { match * value { yaml :: Yaml :: String (ref value) => Ok (Value :: new (uri , ValueKind :: String (value . clone ()))) , yaml :: Yaml :: Real (ref value) => { value . parse :: < f64 > () . map_err (| _ | { Box :: new (FloatParsingError (value . clone ())) as Box < dyn Error + Send + Sync > }) . map (ValueKind :: Float) . map (| f | Value :: new (uri , f)) } yaml :: Yaml :: Integer (value) => Ok (Value :: new (uri , ValueKind :: I64 (value))) , yaml :: Yaml :: Boolean (value) => Ok (Value :: new (uri , ValueKind :: Boolean (value))) , yaml :: Yaml :: Hash (ref table) => { let mut m = Map :: new () ; for (key , value) in table { match key { yaml :: Yaml :: String (k) => m . insert (k . to_owned () , from_yaml_value (uri , value) ?) , yaml :: Yaml :: Integer (k) => m . insert (k . to_string () , from_yaml_value (uri , value) ?) , yaml :: Yaml :: Boolean (k) => m . insert (k . to_string () , from_yaml_value (uri , value) ?) , yaml :: Yaml :: Real (k) => m . insert (k . to_owned () , from_yaml_value (uri , value) ?) , other => Err (Box :: new (UnsupportedHashKeyError (format ! ("{other:?}")))) ? , } ; } Ok (Value :: new (uri , ValueKind :: Table (m))) } yaml :: Yaml :: Array (ref array) => { let mut l = Vec :: new () ; for value in array { l . push (from_yaml_value (uri , value) ?) ; } Ok (Value :: new (uri , ValueKind :: Array (l))) } _ => Ok (Value :: new (uri , ValueKind :: Nil)) , } }
};
}
