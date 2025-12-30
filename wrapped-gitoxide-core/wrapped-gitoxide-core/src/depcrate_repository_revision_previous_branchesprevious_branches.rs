// Generated macro for previous_branches (function)
macro_rules! Depcrate_repository_revision_previous_branchesprevious_branches {
() => {
// Module: crate::repository::revision::previous_branches
// Provides: {"previous_branches"}
// Dependencies: {}
pub fn previous_branches (repo : gix :: Repository , mut out : impl std :: io :: Write , format : OutputFormat ,) -> anyhow :: Result < () > { let branches = repo . head () ? . prior_checked_out_branches () ? . context ("The reflog for HEAD is required") ? ; match format { OutputFormat :: Human => { for (name , id) in branches { writeln ! (out , "{id} {name}") ? ; } } # [cfg (feature = "serde")] OutputFormat :: Json => { serde_json :: to_writer_pretty (& mut out , & branches) ? ; } } Ok (()) }
};
}
