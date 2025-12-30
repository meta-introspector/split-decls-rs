// Generated macro for rename_test_files (function)
macro_rules! Depcrate_rename_lintrename_test_files {
() => {
// Module: crate::rename_lint
// Provides: {"rename_test_files"}
// Dependencies: {}
# [doc = " Renames all test files for the given lint."] # [doc = ""] # [doc = " If `rename_prefixed` is `true` this will also rename tests which have the lint name as a prefix."] fn rename_test_files (old_name : & str , new_name : & str , rename_prefixed : bool) { let mut tests = Vec :: new () ; let mut old_buf = OsString :: from ("tests/ui/") ; let mut new_buf = OsString :: from ("tests/ui/") ; collect_ui_test_names (old_name , rename_prefixed , & mut tests) ; for & (ref name , is_file) in & tests { old_buf . push (name) ; new_buf . extend ([new_name . as_ref () , name . slice_encoded_bytes (old_name . len () ..)]) ; if is_file { try_rename_file (old_buf . as_ref () , new_buf . as_ref ()) ; } else { try_rename_dir (old_buf . as_ref () , new_buf . as_ref ()) ; } old_buf . truncate ("tests/ui/" . len ()) ; new_buf . truncate ("tests/ui/" . len ()) ; } tests . clear () ; old_buf . truncate ("tests/ui" . len ()) ; new_buf . truncate ("tests/ui" . len ()) ; old_buf . push ("-toml/") ; new_buf . push ("-toml/") ; collect_ui_toml_test_names (old_name , rename_prefixed , & mut tests) ; for (name , _) in & tests { old_buf . push (name) ; new_buf . extend ([new_name . as_ref () , name . slice_encoded_bytes (old_name . len () ..)]) ; try_rename_dir (old_buf . as_ref () , new_buf . as_ref ()) ; old_buf . truncate ("tests/ui/" . len ()) ; new_buf . truncate ("tests/ui/" . len ()) ; } }
};
}
