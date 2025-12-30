// Generated macro for function (function)
macro_rules! Depcrate_repository_fsckfunction {
() => {
// Module: crate::repository::fsck
// Provides: {"function"}
// Dependencies: {}
pub fn function (mut repo : gix :: Repository , spec : Option < String > , mut out : impl std :: io :: Write) -> anyhow :: Result < () > { let spec = spec . unwrap_or ("HEAD" . into ()) ; repo . object_cache_size_if_unset (4 * 1024 * 1024) ; repo . objects . refresh_never () ; let id = repo . rev_parse_single (spec . as_str ()) . context ("Only single revisions are supported") ? ; let commits : gix :: revision :: Walk < '_ > = id . object () ? . peel_to_kind (gix :: object :: Kind :: Commit) . context ("Need committish as starting point") ? . id () . ancestors () . all () ? ; let on_missing = | oid : & ObjectId , kind : Kind | { writeln ! (out , "{oid}: {kind}") . expect ("failed to write output") ; } ; let mut check = gix_fsck :: Connectivity :: new (& repo . objects , on_missing) ; for commit in commits { let commit = commit ? ; check . check_commit (& commit . id) ? ; } Ok (()) }
};
}
