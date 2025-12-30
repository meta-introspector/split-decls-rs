// Generated macro for impl_292 (impl)
macro_rules! Depcrate_json_valueimpl_292 {
() => {
// Module: crate::json::value
// Provides: {"impl_292"}
// Dependencies: {}
impl JsonRender for Json { fn render (& self) -> String { match * self { Json :: String (ref s) => s . to_string () , Json :: Bool (i) => i . to_string () , Json :: Number (ref n) => n . to_string () , Json :: Null => String :: new () , Json :: Array (ref a) => { let mut buf = String :: new () ; buf . push ('[') ; for (i , value) in a . iter () . enumerate () { buf . push_str (value . render () . as_ref ()) ; if i < a . len () - 1 { buf . push_str (", ") ; } } buf . push (']') ; buf } Json :: Object (_) => "[object]" . to_owned () , } } }
};
}
