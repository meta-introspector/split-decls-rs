// Generated macro for list (function)
macro_rules! Depcrate_repository_submodulelist {
() => {
// Module: crate::repository::submodule
// Provides: {"list"}
// Dependencies: {}
pub fn list (repo : Repository , mut out : impl std :: io :: Write , format : OutputFormat , dirty_suffix : Option < String > ,) -> anyhow :: Result < () > { if format != OutputFormat :: Human { bail ! ("Only human output is supported for now") } let Some (submodules) = repo . submodules () ? else { return Ok (()) ; } ; for sm in submodules { print_sm (sm , dirty_suffix . as_deref () , & mut out) ? ; } Ok (()) }
};
}
