// Generated macro for format (function)
macro_rules! Depcrateformat {
() => {
// Module: crate
// Provides: {"format"}
// Dependencies: {}
# [doc = " The format function is used as the default value formatter for all values unless the user"] # [doc = " specifies another. It is provided publicly so that it can be called as part of custom formatters."] # [doc = " Values are formatted as follows:"] # [doc = ""] # [doc = " * `Value::Null` => the empty string"] # [doc = " * `Value::Bool` => true|false"] # [doc = " * `Value::Number` => the number, as formatted by `serde_json`."] # [doc = " * `Value::String` => the string, HTML-escaped"] # [doc = ""] # [doc = " Arrays and objects are not formatted, and attempting to do so will result in a rendering error."] pub fn format (value : & Value , output : & mut String) -> Result < () > { match value { Value :: Null => Ok (()) , Value :: Bool (b) => { write ! (output , "{}" , b) ? ; Ok (()) } Value :: Number (n) => { write ! (output , "{}" , n) ? ; Ok (()) } Value :: String (s) => { escape (s , output) ; Ok (()) } _ => Err (unprintable_error ()) , } }
};
}
