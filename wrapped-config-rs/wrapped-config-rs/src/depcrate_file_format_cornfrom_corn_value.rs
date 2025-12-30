// Generated macro for from_corn_value (function)
macro_rules! Depcrate_file_format_cornfrom_corn_value {
() => {
// Module: crate::file::format::corn
// Provides: {"from_corn_value"}
// Dependencies: {}
fn from_corn_value (uri : Option < & String > , value : & corn :: Value < '_ >) -> Value { match value { corn :: Value :: String (value) => Value :: new (uri , ValueKind :: String (value . to_string ())) , corn :: Value :: Integer (value) => Value :: new (uri , ValueKind :: I64 (* value)) , corn :: Value :: Float (value) => Value :: new (uri , ValueKind :: Float (* value)) , corn :: Value :: Boolean (value) => Value :: new (uri , ValueKind :: Boolean (* value)) , corn :: Value :: Object (value) => Value :: new (uri , ValueKind :: Table (value . iter () . map (| (key , value) | (key . to_string () , from_corn_value (uri , value))) . collect () ,) ,) , corn :: Value :: Array (value) => Value :: new (uri , ValueKind :: Array (value . iter () . map (| value | from_corn_value (uri , value)) . collect () ,) ,) , corn :: Value :: Null (_) => Value :: new (uri , ValueKind :: Nil) , } }
};
}
