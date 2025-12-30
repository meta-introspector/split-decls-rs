// Generated macro for collect_ui_test_names (function)
macro_rules! Depcrate_rename_lintcollect_ui_test_names {
() => {
// Module: crate::rename_lint
// Provides: {"collect_ui_test_names"}
// Dependencies: {}
fn collect_ui_test_names (lint : & str , rename_prefixed : bool , dst : & mut Vec < (OsString , bool) >) { for e in fs :: read_dir ("tests/ui") . expect ("error reading `tests/ui`") { let e = e . expect ("error reading `tests/ui`") ; let name = e . file_name () ; if let Some ((name_only , _)) = name . as_encoded_bytes () . split_once (| & x | x == b'.') { if name_only . starts_with (lint . as_bytes ()) && (rename_prefixed || name_only . len () == lint . len ()) { dst . push ((name , true)) ; } } else if name . as_encoded_bytes () . starts_with (lint . as_bytes ()) && (rename_prefixed || name . len () == lint . len ()) { dst . push ((name , false)) ; } } }
};
}
