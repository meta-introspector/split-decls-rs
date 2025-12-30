// Generated macro for collect_parents (function)
macro_rules! Depcrate_commit_simplecollect_parents {
() => {
// Module: crate::commit::simple
// Provides: {"collect_parents"}
// Dependencies: {}
fn collect_parents (dest : & mut SmallVec < (gix_hash :: ObjectId , gix_date :: SecondsSinceUnixEpoch) , 2 > , cache : Option < & gix_commitgraph :: Graph > , parents : gix_commitgraph :: file :: commit :: Parents < '_ > ,) -> bool { dest . clear () ; let cache = cache . as_ref () . expect ("parents iter is available, backed by `cache`") ; for parent_id in parents { match parent_id { Ok (pos) => dest . push ({ let parent = cache . commit_at (pos) ; (parent . id () . to_owned () , parent . committer_timestamp () as gix_date :: SecondsSinceUnixEpoch ,) }) , Err (_err) => return false , } } true }
};
}
