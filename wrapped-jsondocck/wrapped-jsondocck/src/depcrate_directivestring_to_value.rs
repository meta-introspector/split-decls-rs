// Generated macro for string_to_value (function)
macro_rules! Depcrate_directivestring_to_value {
() => {
// Module: crate::directive
// Provides: {"string_to_value"}
// Dependencies: {}
fn string_to_value < 'a > (s : & str , cache : & 'a Cache) -> Cow < 'a , Value > { if s . starts_with ("$") { Cow :: Borrowed (& cache . variables . get (& s [1 ..]) . unwrap_or_else (| | { panic ! ("No variable: `{}`. Current state: `{:?}`" , & s [1 ..] , cache . variables) })) } else { Cow :: Owned (serde_json :: from_str (s) . expect (& format ! ("Cannot convert `{}` to json" , s))) } }
};
}
