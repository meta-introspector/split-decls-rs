// Generated macro for collect_ui_toml_test_names (function)
macro_rules! Depcrate_rename_lintcollect_ui_toml_test_names {
() => {
// Module: crate::rename_lint
// Provides: {"collect_ui_toml_test_names"}
// Dependencies: {}
fn collect_ui_toml_test_names (lint : & str , rename_prefixed : bool , dst : & mut Vec < (OsString , bool) >) { if rename_prefixed { for e in fs :: read_dir ("tests/ui-toml") . expect ("error reading `tests/ui-toml`") { let e = e . expect ("error reading `tests/ui-toml`") ; let name = e . file_name () ; if name . as_encoded_bytes () . starts_with (lint . as_bytes ()) && e . file_type () . is_ok_and (| ty | ty . is_dir ()) { dst . push ((name , false)) ; } } } else { dst . push ((lint . into () , false)) ; } }
};
}
