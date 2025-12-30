// Generated macro for impl_564 (impl)
macro_rules! Depcrate_uriimpl_564 {
() => {
// Module: crate::uri
// Provides: {"impl_564"}
// Dependencies: {}
impl PartialEq < str > for Uri { fn eq (& self , other : & str) -> bool { let mut other = other . as_bytes () ; let mut absolute = false ; if let Some (scheme) = self . scheme () { let scheme = scheme . as_str () . as_bytes () ; absolute = true ; if other . len () < scheme . len () + 3 { return false ; } if ! scheme . eq_ignore_ascii_case (& other [.. scheme . len ()]) { return false ; } other = & other [scheme . len () ..] ; if & other [.. 3] != b"://" { return false ; } other = & other [3 ..] ; } if let Some (auth) = self . authority () { let len = auth . data . len () ; absolute = true ; if other . len () < len { return false ; } if ! auth . data . as_bytes () . eq_ignore_ascii_case (& other [.. len]) { return false ; } other = & other [len ..] ; } let path = self . path () ; if other . len () < path . len () || path . as_bytes () != & other [.. path . len ()] { if absolute && path == "/" { } else { return false ; } } else { other = & other [path . len () ..] ; } if let Some (query) = self . query () { if other . is_empty () { return query . is_empty () ; } if other [0] != b'?' { return false ; } other = & other [1 ..] ; if other . len () < query . len () { return false ; } if query . as_bytes () != & other [.. query . len ()] { return false ; } other = & other [query . len () ..] ; } other . is_empty () || other [0] == b'#' } }
};
}
