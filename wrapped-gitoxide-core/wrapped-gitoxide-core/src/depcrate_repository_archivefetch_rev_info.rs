// Generated macro for fetch_rev_info (function)
macro_rules! Depcrate_repository_archivefetch_rev_info {
() => {
// Module: crate::repository::archive
// Provides: {"fetch_rev_info"}
// Dependencies: {}
fn fetch_rev_info (object : gix :: Object < '_ > ,) -> anyhow :: Result < (Option < gix :: date :: SecondsSinceUnixEpoch > , gix :: ObjectId) > { Ok (match object . kind { gix :: object :: Kind :: Commit => { let commit = object . into_commit () ; (Some (commit . committer () ? . seconds ()) , commit . tree_id () ? . detach ()) } gix :: object :: Kind :: Tree => (None , object . id) , gix :: object :: Kind :: Tag => fetch_rev_info (object . peel_to_kind (gix :: object :: Kind :: Commit) ?) ? , gix :: object :: Kind :: Blob => bail ! ("Cannot derive commit or tree from blob at {}" , object . id) , }) }
};
}
