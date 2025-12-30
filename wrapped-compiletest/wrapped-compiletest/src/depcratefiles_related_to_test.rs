// Generated macro for files_related_to_test (function)
macro_rules! Depcratefiles_related_to_test {
() => {
// Module: crate
// Provides: {"files_related_to_test"}
// Dependencies: {}
# [doc = " Returns a list of files that, if modified, would cause this test to no"] # [doc = " longer be up-to-date."] # [doc = ""] # [doc = " (Might be inaccurate in some cases.)"] fn files_related_to_test (config : & Config , testpaths : & TestPaths , props : & EarlyProps , revision : Option < & str > ,) -> Vec < Utf8PathBuf > { let mut related = vec ! [] ; if testpaths . file . is_dir () { for entry in WalkDir :: new (& testpaths . file) { let path = entry . unwrap () . into_path () ; if path . is_file () { related . push (Utf8PathBuf :: try_from (path) . unwrap ()) ; } } } else { related . push (testpaths . file . clone ()) ; } for aux in props . aux . all_aux_path_strings () { let path = testpaths . file . parent () . unwrap () . join ("auxiliary") . join (aux) ; related . push (path) ; } for extension in UI_EXTENSIONS { let path = expected_output_path (testpaths , revision , & config . compare_mode , extension) ; related . push (path) ; } related . push (config . src_root . join ("tests") . join ("auxiliary") . join ("minicore.rs")) ; related }
};
}
