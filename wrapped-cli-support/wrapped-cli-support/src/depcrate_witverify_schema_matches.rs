// Generated macro for verify_schema_matches (function)
macro_rules! Depcrate_witverify_schema_matches {
() => {
// Module: crate::wit
// Provides: {"verify_schema_matches"}
// Dependencies: {}
fn verify_schema_matches (data : & [u8]) -> Result < Option < & str > , Error > { macro_rules ! bad { () => { bail ! ("failed to decode what looked like wasm-bindgen data") } ; } let data = match str :: from_utf8 (data) { Ok (s) => s , Err (_) => bad ! () , } ; log :: debug ! ("found version specifier {data}") ; if ! data . starts_with ('{') || ! data . ends_with ('}') { bad ! () } let needle = "\"schema_version\":\"" ; let rest = match data . find (needle) { Some (i) => & data [i + needle . len () ..] , None => bad ! () , } ; let their_schema_version = match rest . find ('"') { Some (i) => & rest [.. i] , None => bad ! () , } ; if their_schema_version == wasm_bindgen_shared :: SCHEMA_VERSION { return Ok (None) ; } let needle = "\"version\":\"" ; let rest = match data . find (needle) { Some (i) => & data [i + needle . len () ..] , None => bad ! () , } ; let their_version = match rest . find ('"') { Some (i) => & rest [.. i] , None => bad ! () , } ; Ok (Some (their_version)) }
};
}
