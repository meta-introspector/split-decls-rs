// Generated macro for tests (module)
macro_rules! Depcrate_into_urltests {
() => {
// Module: crate::into_url
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: error :: Error ; # [test] fn into_url_file_scheme () { let err = "file:///etc/hosts" . into_url () . unwrap_err () ; assert_eq ! (err . source () . unwrap () . to_string () , "URL scheme is not allowed") ; } # [test] fn into_url_blob_scheme () { let err = "blob:https://example.com" . into_url () . unwrap_err () ; assert_eq ! (err . source () . unwrap () . to_string () , "URL scheme is not allowed") ; } if_wasm ! { use wasm_bindgen_test ::*; # [wasm_bindgen_test] fn into_url_blob_scheme_wasm () { let url = "blob:http://example.com" . into_url () . unwrap () ; assert_eq ! (url . as_str () , "blob:http://example.com") ; } } }
};
}
