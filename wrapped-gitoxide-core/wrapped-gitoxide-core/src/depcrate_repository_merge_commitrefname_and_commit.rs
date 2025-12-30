// Generated macro for refname_and_commit (function)
macro_rules! Depcrate_repository_merge_commitrefname_and_commit {
() => {
// Module: crate::repository::merge::commit
// Provides: {"refname_and_commit"}
// Dependencies: {}
fn refname_and_commit (repo : & gix :: Repository , revspec : BString ,) -> anyhow :: Result < (Option < BString > , gix :: hash :: ObjectId) > { let spec = repo . rev_parse (revspec . as_bstr ()) ? ; let commit_id = spec . single () . context ("Expected revspec to expand to a single rev only") ? . object () ? . peel_to_commit () ? . id ; let refname = spec . first_reference () . map (| r | r . name . shorten () . as_bstr () . to_owned ()) ; Ok ((refname , commit_id)) }
};
}
