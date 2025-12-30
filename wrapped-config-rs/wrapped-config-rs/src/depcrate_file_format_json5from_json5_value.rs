// Generated macro for from_json5_value (function)
macro_rules! Depcrate_file_format_json5from_json5_value {
() => {
// Module: crate::file::format::json5
// Provides: {"from_json5_value"}
// Dependencies: {}
fn from_json5_value (uri : Option < & String > , value : Val) -> Value { let vk = match value { Val :: Null => ValueKind :: Nil , Val :: String (v) => ValueKind :: String (v) , Val :: Integer (v) => ValueKind :: I64 (v) , Val :: Float (v) => ValueKind :: Float (v) , Val :: Boolean (v) => ValueKind :: Boolean (v) , Val :: Object (table) => { let m = table . into_iter () . map (| (k , v) | (k , from_json5_value (uri , v))) . collect () ; ValueKind :: Table (m) } Val :: Array (array) => { let l = array . into_iter () . map (| v | from_json5_value (uri , v)) . collect () ; ValueKind :: Array (l) } } ; Value :: new (uri , vk) }
};
}
