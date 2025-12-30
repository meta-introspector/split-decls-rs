// Generated macro for canonical_value (function)
macro_rules! Depcrate_value_canonicalcanonical_value {
() => {
// Module: crate::value::canonical
// Provides: {"canonical_value"}
// Dependencies: {}
# [doc = " Recursively convert a Value to its canonical form as defined in RFC 8949 \"core deterministic encoding requirements\"."] pub fn canonical_value (value : Value) -> Value { match value { Value :: Map (entries) => { let mut canonical_entries : Vec < (Value , Value) > = entries . into_iter () . map (| (k , v) | (canonical_value (k) , canonical_value (v))) . collect () ; canonical_entries . sort_by (| (k1 , _) , (k2 , _) | cmp_value (k1 , k2)) ; Value :: Map (canonical_entries) } Value :: Array (elements) => { let canonical_elements : Vec < Value > = elements . into_iter () . map (canonical_value) . collect () ; Value :: Array (canonical_elements) } Value :: Tag (tag , inner_value) => { Value :: Tag (tag , Box :: new (canonical_value (* inner_value))) } _ => value , } }
};
}
