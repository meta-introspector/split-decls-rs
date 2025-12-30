// Generated macro for push_filter (function)
macro_rules! Depcrate_filterpush_filter {
() => {
// Module: crate::filter
// Provides: {"push_filter"}
// Dependencies: {}
# [track_caller] fn push_filter (reader : & Reader , rules : & mut Vec < (String , bool) > , filter : & str , include : bool) { if reader . contains_key (filter) { rules . push ((filter . to_string () , include)) ; return ; } if let Some ((namespace , name)) = filter . rsplit_once ('.') { if reader . with_full_name (namespace , name) . next () . is_some () { rules . push ((filter . to_string () , include)) ; return ; } if let Some (starts_with) = name . strip_suffix ('*') { if let Some (types) = reader . get (namespace) { let prev_len = rules . len () ; for name in types . keys () { if name . starts_with (starts_with) { rules . push ((format ! ("{namespace}.{name}") , include)) ; } } if prev_len != rules . len () { return ; } } } } let mut pushed = false ; for (namespace , types) in reader . iter () { if types . get (filter) . is_some () { rules . push ((format ! ("{namespace}.{filter}") , include)) ; pushed = true ; } } if pushed { return ; } if reader . keys () . any (| namespace | namespace_starts_with (namespace , filter)) { rules . push ((filter . to_string () , include)) ; return ; } panic ! ("type not found: `{filter}`") ; }
};
}
