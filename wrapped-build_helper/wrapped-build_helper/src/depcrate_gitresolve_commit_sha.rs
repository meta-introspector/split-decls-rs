// Generated macro for resolve_commit_sha (function)
macro_rules! Depcrate_gitresolve_commit_sha {
() => {
// Module: crate::git
// Provides: {"resolve_commit_sha"}
// Dependencies: {}
# [doc = " Resolve the commit SHA of `commit_ref`."] fn resolve_commit_sha (git_dir : Option < & Path > , commit_ref : & str) -> Result < String , String > { let mut git = Command :: new ("git") ; if let Some (git_dir) = git_dir { git . current_dir (git_dir) ; } git . args (["rev-parse" , commit_ref]) ; Ok (output_result (& mut git) ? . trim () . to_owned ()) }
};
}
