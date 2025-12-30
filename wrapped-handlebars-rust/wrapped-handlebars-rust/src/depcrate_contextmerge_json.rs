// Generated macro for merge_json (function)
macro_rules! Depcrate_contextmerge_json {
() => {
// Module: crate::context
// Provides: {"merge_json"}
// Dependencies: {}
pub (crate) fn merge_json (base : & Json , addition : & HashMap < & str , & Json >) -> Json { if addition . is_empty () { return base . clone () ; } let mut base_map = match base { Json :: Object (ref m) => m . clone () , Json :: Array (ref a) => { let mut base_map = Map :: new () ; for (idx , value) in a . iter () . enumerate () { base_map . insert (idx . to_string () , value . clone ()) ; } base_map } Json :: String (ref s) => { let mut base_map = Map :: new () ; for (idx , value) in s . chars () . enumerate () { base_map . insert (idx . to_string () , Json :: String (value . to_string ())) ; } base_map } _ => Map :: new () , } ; for (k , v) in addition { base_map . insert ((* k) . to_string () , (* v) . clone ()) ; } Json :: Object (base_map) }
};
}
