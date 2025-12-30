// Generated macro for get_closest_upstream_commit (function)
macro_rules! Depcrate_gitget_closest_upstream_commit {
() => {
// Module: crate::git
// Provides: {"get_closest_upstream_commit"}
// Dependencies: {}
# [doc = " Returns the most recent (ordered chronologically) commit found in the local history that"] # [doc = " should exist upstream. We identify upstream commits by the e-mail of the commit"] # [doc = " author."] # [doc = ""] # [doc = " If we are in CI, we simply return our first parent."] pub fn get_closest_upstream_commit (git_dir : Option < & Path > , config : & GitConfig < '_ > , env : CiEnv ,) -> Result < Option < String > , String > { let base = match env { CiEnv :: None => "HEAD" , CiEnv :: GitHubActions => { return resolve_commit_sha (git_dir , "HEAD^1") . map (Some) ; } } ; let mut git = Command :: new ("git") ; if let Some (git_dir) = git_dir { git . current_dir (git_dir) ; } git . args (["rev-list" , "--author-date-order" , & format ! ("--author={}" , config . git_merge_commit_email) , "-n1" , base ,]) ; let output = output_result (& mut git) ? . trim () . to_owned () ; if output . is_empty () { Ok (None) } else { Ok (Some (output)) } }
};
}
