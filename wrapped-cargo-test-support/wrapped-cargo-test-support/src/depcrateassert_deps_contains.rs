// Generated macro for assert_deps_contains (function)
macro_rules! Depcrateassert_deps_contains {
() => {
// Module: crate
// Provides: {"assert_deps_contains"}
// Dependencies: {}
# [track_caller] pub fn assert_deps_contains (project : & Project , fingerprint : & str , expected : & [(u8 , & str)]) { assert_deps (project , fingerprint , | info_path , entries | { for (e_kind , e_path) in expected { let pattern = glob :: Pattern :: new (e_path) . unwrap () ; let count = entries . iter () . filter (| (kind , path) | kind == e_kind && pattern . matches (path)) . count () ; if count != 1 { panic ! ("Expected 1 match of {} {} in {:?}, got {}:\n{:#?}" , e_kind , e_path , info_path , count , entries) ; } } }) }
};
}
