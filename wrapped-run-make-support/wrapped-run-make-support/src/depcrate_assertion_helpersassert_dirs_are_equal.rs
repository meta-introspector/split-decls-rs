// Generated macro for assert_dirs_are_equal (function)
macro_rules! Depcrate_assertion_helpersassert_dirs_are_equal {
() => {
// Module: crate::assertion_helpers
// Provides: {"assert_dirs_are_equal"}
// Dependencies: {}
# [doc = " Assert that all files in `dir1` exist and have the same content in `dir2`"] pub fn assert_dirs_are_equal (dir1 : impl AsRef < Path > , dir2 : impl AsRef < Path >) { let dir2 = dir2 . as_ref () ; fs :: read_dir_entries (dir1 , | entry_path | { let entry_name = entry_path . file_name () . unwrap () ; if entry_path . is_dir () { assert_dirs_are_equal (& entry_path , & dir2 . join (entry_name)) ; } else { let path2 = dir2 . join (entry_name) ; let file1 = fs :: read (& entry_path) ; let file2 = fs :: read (& path2) ; assert ! (file1 == file2 , "`{}` and `{}` have different content" , entry_path . display () , path2 . display () ,) ; } }) ; }
};
}
