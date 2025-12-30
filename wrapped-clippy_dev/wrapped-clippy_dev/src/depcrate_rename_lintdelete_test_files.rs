// Generated macro for delete_test_files (function)
macro_rules! Depcrate_rename_lintdelete_test_files {
() => {
// Module: crate::rename_lint
// Provides: {"delete_test_files"}
// Dependencies: {}
fn delete_test_files (lint : & str , rename_prefixed : bool) { let mut tests = Vec :: new () ; let mut buf = OsString :: from ("tests/ui/") ; collect_ui_test_names (lint , rename_prefixed , & mut tests) ; for & (ref name , is_file) in & tests { buf . push (name) ; if is_file { delete_file_if_exists (buf . as_ref ()) ; } else { delete_dir_if_exists (buf . as_ref ()) ; } buf . truncate ("tests/ui/" . len ()) ; } buf . truncate ("tests/ui" . len ()) ; buf . push ("-toml/") ; tests . clear () ; collect_ui_toml_test_names (lint , rename_prefixed , & mut tests) ; for (name , _) in & tests { buf . push (name) ; delete_dir_if_exists (buf . as_ref ()) ; buf . truncate ("tests/ui/" . len ()) ; } }
};
}
