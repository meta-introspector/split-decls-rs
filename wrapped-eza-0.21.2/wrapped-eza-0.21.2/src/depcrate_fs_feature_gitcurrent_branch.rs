// Generated macro for current_branch (function)
macro_rules! Depcrate_fs_feature_gitcurrent_branch {
() => {
// Module: crate::fs::feature::git
// Provides: {"current_branch"}
// Dependencies: {}
fn current_branch (repo : & git2 :: Repository) -> Option < String > { let head = match repo . head () { Ok (head) => Some (head) , Err (ref e) if e . code () == git2 :: ErrorCode :: UnbornBranch || e . code () == git2 :: ErrorCode :: NotFound => { return None } Err (e) => { error ! ("Error looking up Git branch: {:?}" , e) ; return None ; } } ; head . and_then (| h | h . shorthand () . map (std :: string :: ToString :: to_string)) }
};
}
