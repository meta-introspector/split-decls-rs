// Generated macro for parse_attrs (function)
macro_rules! Depcrate_msrvsparse_attrs {
() => {
// Module: crate::msrvs
// Provides: {"parse_attrs"}
// Dependencies: {}
fn parse_attrs (sess : & Session , attrs : & [impl AttributeExt]) -> Option < RustcVersion > { let mut msrv_attrs = attrs . iter () . filter (| attr | attr . path_matches (& [sym :: clippy , sym :: msrv])) ; let msrv_attr = msrv_attrs . next () ? ; if let Some (duplicate) = msrv_attrs . next_back () { sess . dcx () . struct_span_err (duplicate . span () , "`clippy::msrv` is defined multiple times") . with_span_note (msrv_attr . span () , "first definition found here") . emit () ; } let Some (msrv) = msrv_attr . value_str () else { sess . dcx () . span_err (msrv_attr . span () , "bad clippy attribute") ; return None ; } ; let Some (version) = parse_version (msrv) else { sess . dcx () . span_err (msrv_attr . span () , format ! ("`{msrv}` is not a valid Rust version")) ; return None ; } ; Some (version) }
};
}
