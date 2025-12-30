// Generated macro for extract_root_table (function)
macro_rules! Depcrate_formatextract_root_table {
() => {
// Module: crate::format
// Provides: {"extract_root_table"}
// Dependencies: {}
pub (crate) fn extract_root_table (uri : Option < & String > , value : Value ,) -> Result < Map < String , Value > , Box < dyn Error + Send + Sync > > { match value . kind { ValueKind :: Table (map) => Ok (map) , ValueKind :: Nil => Err (Unexpected :: Unit) , ValueKind :: Array (_value) => Err (Unexpected :: Seq) , ValueKind :: Boolean (value) => Err (Unexpected :: Bool (value)) , ValueKind :: I64 (value) => Err (Unexpected :: I64 (value)) , ValueKind :: I128 (value) => Err (Unexpected :: I128 (value)) , ValueKind :: U64 (value) => Err (Unexpected :: U64 (value)) , ValueKind :: U128 (value) => Err (Unexpected :: U128 (value)) , ValueKind :: Float (value) => Err (Unexpected :: Float (value)) , ValueKind :: String (value) => Err (Unexpected :: Str (value)) , } . map_err (| err | ConfigError :: invalid_root (uri , err)) . map_err (| err | Box :: new (err) as Box < dyn Error + Send + Sync >) }
};
}
