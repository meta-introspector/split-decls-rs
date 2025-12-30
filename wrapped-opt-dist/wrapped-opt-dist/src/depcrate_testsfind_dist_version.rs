// Generated macro for find_dist_version (function)
macro_rules! Depcrate_testsfind_dist_version {
() => {
// Module: crate::tests
// Provides: {"find_dist_version"}
// Dependencies: {}
# [doc = " Tries to find the version of the dist artifacts (either nightly, beta, or 1.XY.Z)."] fn find_dist_version (directory : & Utf8Path) -> anyhow :: Result < String > { let archive = find_file_in_dir (directory , "reproducible-artifacts-" , ".tar.xz") ? . file_name () . unwrap () . to_string () ; let (version , _) = archive . strip_prefix ("reproducible-artifacts-") . unwrap () . split_once ('-') . unwrap () ; Ok (version . to_string ()) }
};
}
