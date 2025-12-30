// Generated macro for modified_tests (function)
macro_rules! Depcratemodified_tests {
() => {
// Module: crate
// Provides: {"modified_tests"}
// Dependencies: {}
# [doc = " Returns a list of modified/untracked test files that should be run when"] # [doc = " the `--only-modified` flag is in use."] # [doc = ""] # [doc = " (Might be inaccurate in some cases.)"] fn modified_tests (config : & Config , dir : & Utf8Path) -> Result < Vec < Utf8PathBuf > , String > { if ! config . only_modified { return Ok (vec ! []) ; } let files = get_git_modified_files (& config . git_config () , Some (dir . as_std_path ()) , & vec ! ["rs" , "stderr" , "fixed"] ,) ? ; let untracked_files = get_git_untracked_files (Some (dir . as_std_path ())) ? . unwrap_or (vec ! []) ; let all_paths = [& files [..] , & untracked_files [..]] . concat () ; let full_paths = { let mut full_paths : Vec < Utf8PathBuf > = all_paths . into_iter () . map (| f | Utf8PathBuf :: from (f) . with_extension ("") . with_extension ("rs")) . filter_map (| f | if Utf8Path :: new (& f) . exists () { f . canonicalize_utf8 () . ok () } else { None } ,) . collect () ; full_paths . dedup () ; full_paths . sort_unstable () ; full_paths } ; Ok (full_paths) }
};
}
