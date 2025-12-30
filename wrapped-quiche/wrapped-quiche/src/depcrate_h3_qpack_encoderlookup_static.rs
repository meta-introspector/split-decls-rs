// Generated macro for lookup_static (function)
macro_rules! Depcrate_h3_qpack_encoderlookup_static {
() => {
// Module: crate::h3::qpack::encoder
// Provides: {"lookup_static"}
// Dependencies: {}
fn lookup_static < T : NameValue > (h : & T) -> Option < (u64 , bool) > { let table_for_len = super :: static_table :: STATIC_ENCODE_TABLE . get (h . name () . len ()) ? ; let cmp_lowercase = | a : & [u8] , b : & [u8] | { std :: iter :: zip (a , b) . all (| (a , b) | a . eq (& b . to_ascii_lowercase ())) } ; for (name , values) in table_for_len . iter () { if cmp_lowercase (name , h . name ()) { for (value , enc) in values . iter () { if value . is_empty () { return Some ((* enc , false)) ; } if h . value () == * value { return Some ((* enc , true)) ; } } return Some ((values . first () ? . 1 , false)) ; } } None }
};
}
