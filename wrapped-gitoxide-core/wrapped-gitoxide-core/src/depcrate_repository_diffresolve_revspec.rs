// Generated macro for resolve_revspec (function)
macro_rules! Depcrate_repository_diffresolve_revspec {
() => {
// Module: crate::repository::diff
// Provides: {"resolve_revspec"}
// Dependencies: {}
fn resolve_revspec (repo : & gix :: Repository , revspec : BString ,) -> Result < (ObjectId , Option < std :: path :: PathBuf > , BString) , anyhow :: Error > { let result = repo . rev_parse (revspec . as_bstr ()) ; match result { Err (gix :: revision :: spec :: parse :: Error :: FindReference (gix :: refs :: file :: find :: existing :: Error :: NotFound { name , })) => { let root = repo . workdir () . map (ToOwned :: to_owned) ; let name = gix :: path :: os_string_into_bstring (name . into ()) ? ; Ok ((ObjectId :: null (gix :: hash :: Kind :: Sha1) , root , name)) } Err (err) => Err (err . into ()) , Ok (resolved_revspec) => { let blob_id = resolved_revspec . single () . context (format ! ("rev-spec '{revspec}' must resolve to a single object")) ? ; let (path , _) = resolved_revspec . path_and_mode () . context (format ! ("rev-spec '{revspec}' must contain a path")) ? ; Ok ((blob_id . into () , None , path . into ())) } } }
};
}
