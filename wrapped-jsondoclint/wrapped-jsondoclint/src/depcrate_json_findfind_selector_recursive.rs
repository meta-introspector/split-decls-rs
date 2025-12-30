// Generated macro for find_selector_recursive (function)
macro_rules! Depcrate_json_findfind_selector_recursive {
() => {
// Module: crate::json_find
// Provides: {"find_selector_recursive"}
// Dependencies: {}
fn find_selector_recursive (haystack : & Value , needle : & Value , result : & mut Vec < Selector > , pos : & mut Selector ,) { if needle == haystack { result . push (pos . clone ()) ; } else { match haystack { Value :: Null => { } Value :: Bool (_) => { } Value :: Number (_) => { } Value :: String (_) => { } Value :: Array (arr) => { for (idx , subhaystack) in arr . iter () . enumerate () { pos . push (SelectorPart :: Index (idx)) ; find_selector_recursive (subhaystack , needle , result , pos) ; pos . pop () . unwrap () ; } } Value :: Object (obj) => { for (key , subhaystack) in obj { pos . push (SelectorPart :: Field (key . clone ())) ; find_selector_recursive (subhaystack , needle , result , pos) ; pos . pop () . unwrap () ; } } } } }
};
}
