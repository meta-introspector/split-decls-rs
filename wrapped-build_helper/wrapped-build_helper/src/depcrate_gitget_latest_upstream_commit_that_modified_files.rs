// Generated macro for get_latest_upstream_commit_that_modified_files (function)
macro_rules! Depcrate_gitget_latest_upstream_commit_that_modified_files {
() => {
// Module: crate::git
// Provides: {"get_latest_upstream_commit_that_modified_files"}
// Dependencies: {}
# [doc = " Returns the latest upstream commit that modified `target_paths`, or `None` if no such commit"] # [doc = " was found."] fn get_latest_upstream_commit_that_modified_files (git_dir : & Path , git_config : & GitConfig < '_ > , target_paths : & [& str] ,) -> Result < Option < String > , String > { let mut git = Command :: new ("git") ; git . current_dir (git_dir) ; let upstream = get_closest_upstream_commit (Some (git_dir) , git_config , CiEnv :: None) ? . unwrap_or_else (| | "HEAD" . to_string ()) ; git . args (["rev-list" , "--first-parent" , "-n1" , & upstream , "--author" , git_config . git_merge_commit_email ,]) ; if ! target_paths . is_empty () { git . arg ("--") . args (target_paths) ; } let output = output_result (& mut git) ? . trim () . to_owned () ; if output . is_empty () { Ok (None) } else { Ok (Some (output)) } }
};
}
