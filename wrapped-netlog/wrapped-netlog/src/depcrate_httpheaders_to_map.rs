// Generated macro for headers_to_map (function)
macro_rules! Depcrate_httpheaders_to_map {
() => {
// Module: crate::http
// Provides: {"headers_to_map"}
// Dependencies: {}
pub fn headers_to_map (hdrs : & [String]) -> BTreeMap < String , String > { let mut ret = BTreeMap :: new () ; for hdr in hdrs { let mut split = hdr . split (": ") ; let name = split . next () ; let val = split . next () ; match (name , val) { (Some (k) , Some (v)) => { ret . insert (k . to_string () , v . to_string ()) ; } , (Some (k) , None) => { ret . insert (k . to_string () , "" . to_string ()) ; } , _ => () , } } ret }
};
}
