// Generated macro for is_metadata_outdated (function)
macro_rules! Depcrate_serveis_metadata_outdated {
() => {
// Module: crate::serve
// Provides: {"is_metadata_outdated"}
// Dependencies: {}
fn is_metadata_outdated (time : SystemTime) -> bool { if time < mtime ("util/gh-pages/index_template.html") || time < mtime ("tests/compile-test.rs") { return true ; } let Some (dir) = log_err_and_continue (fs :: read_dir (".") , "." . as_ref ()) else { return false ; } ; dir . map_while (| e | log_err_and_continue (e , "." . as_ref ())) . any (| e | { let name = e . file_name () ; let name_bytes = name . as_encoded_bytes () ; if (name_bytes . starts_with (b"clippy_lints") && name_bytes != b"clippy_lints_internal") || name_bytes == b"clippy_config" { WalkDir :: new (& name) . into_iter () . map_while (| e | log_err_and_continue (e , name . as_ref ())) . filter (| e | e . file_type () . is_file ()) . filter_map (| e | { log_err_and_continue (e . metadata () , e . path ()) . and_then (| m | log_err_and_continue (m . modified () , e . path ())) }) . any (| ftime | time < ftime) } else { false } }) }
};
}
