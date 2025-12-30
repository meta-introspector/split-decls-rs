// Generated macro for prepare (function)
macro_rules! Depcrate_queryprepare {
() => {
// Module: crate::query
// Provides: {"prepare"}
// Dependencies: {}
pub fn prepare (repo_dir : & std :: path :: Path , mut progress : impl gix :: NestedProgress , err : impl std :: io :: Write , opts : Options ,) -> anyhow :: Result < Engine > { let repo = gix :: discover (repo_dir) ? ; let mut con = db :: create (repo . git_dir () . join ("ein.query")) ? ; let commits = engine :: update (& repo , & mut con , & mut progress , err , opts) ? ; Ok (Engine { repo , con , commits }) }
};
}
