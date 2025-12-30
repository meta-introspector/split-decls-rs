// Generated macro for remove_files_callback (function)
macro_rules! Depcrate_testremove_files_callback {
() => {
// Module: crate::test
// Provides: {"remove_files_callback"}
// Dependencies: {}
fn remove_files_callback < 'a > (file_path : & 'a str , test_type : & 'a str ,) -> impl Fn (& Path) -> Result < bool , String > + 'a { move | rust_path | { let files = std :: fs :: read_to_string (file_path) . unwrap_or_default () ; let first_file_name = files . lines () . next () . unwrap_or ("") ; if first_file_name . ends_with ('/') { if let Ok (files) = std :: fs :: read_to_string (file_path) { for file in files . split ('\n') . map (| line | line . trim ()) . filter (| line | ! line . is_empty ()) { let path = rust_path . join (file) ; if let Err (e) = remove_dir_all (& path) { println ! ("Failed to remove directory `{}`: {}" , path . display () , e) ; } } } else { println ! ("Failed to read `{file_path}`, not putting back failing {test_type} tests") ; } } else { if let Ok (files) = std :: fs :: read_to_string (file_path) { for file in files . split ('\n') . map (| line | line . trim ()) . filter (| line | ! line . is_empty ()) { let path = rust_path . join (file) ; remove_file (& path) ? ; } } else { println ! ("Failed to read `{file_path}`, not putting back failing ui tests") ; } } Ok (true) } }
};
}
