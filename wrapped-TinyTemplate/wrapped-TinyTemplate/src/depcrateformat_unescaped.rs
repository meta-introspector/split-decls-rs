// Generated macro for format_unescaped (function)
macro_rules! Depcrateformat_unescaped {
() => {
// Module: crate
// Provides: {"format_unescaped"}
// Dependencies: {}
# [doc = " Identical to [`format`](fn.format.html) except that this does not perform HTML escaping."] pub fn format_unescaped (value : & Value , output : & mut String) -> Result < () > { match value { Value :: Null => Ok (()) , Value :: Bool (b) => { write ! (output , "{}" , b) ? ; Ok (()) } Value :: Number (n) => { write ! (output , "{}" , n) ? ; Ok (()) } Value :: String (s) => { output . push_str (s) ; Ok (()) } _ => Err (unprintable_error ()) , } }
};
}
