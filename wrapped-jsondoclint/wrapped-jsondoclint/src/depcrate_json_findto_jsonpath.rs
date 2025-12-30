// Generated macro for to_jsonpath (function)
macro_rules! Depcrate_json_findto_jsonpath {
() => {
// Module: crate::json_find
// Provides: {"to_jsonpath"}
// Dependencies: {}
pub fn to_jsonpath (sel : & Selector) -> String { let mut s = String :: from ("$") ; for part in sel { match part { SelectorPart :: Field (name) => { if is_jsonpath_safe (name) { write ! (& mut s , ".{}" , name) . unwrap () ; } else { write ! (& mut s , "[{name:?}]") . unwrap () ; } } SelectorPart :: Index (idx) => write ! (& mut s , "[{idx}]") . unwrap () , } } s }
};
}
