// Generated macro for collect_parents (function)
macro_rules! Depcrate_commit_topo_itercollect_parents {
() => {
// Module: crate::commit::topo::iter
// Provides: {"collect_parents"}
// Dependencies: {}
fn collect_parents < Find > (cache : & mut Option < gix_commitgraph :: Graph > , f : Find , id : & oid , first_only : bool , buf : & mut Vec < u8 > ,) -> Result < SmallVec < (ObjectId , GenAndCommitTime) , 1 > , Error > where Find : gix_object :: Find , { let mut parents = SmallVec :: < (ObjectId , GenAndCommitTime) , 1 > :: new () ; match find (cache . as_ref () , & f , id , buf) ? { Either :: CommitRefIter (c) => { for token in c { use gix_object :: commit :: ref_iter :: Token as T ; match token { Ok (T :: Tree { .. }) => continue , Ok (T :: Parent { id }) => { parents . push ((id , (0 , 0))) ; if first_only { break ; } } Ok (_past_parents) => break , Err (err) => return Err (err . into ()) , } } for (id , gen_time) in parents . iter_mut () { let commit = find (cache . as_ref () , & f , id , buf) ? ; * gen_time = gen_and_commit_time (commit) ? ; } } Either :: CachedCommit (c) => { for pos in c . iter_parents () { let Ok (pos) = pos else { * cache = None ; return collect_parents (cache , f , id , first_only , buf) ; } ; let parent_commit = cache . as_ref () . expect ("cache exists if CachedCommit was returned") . commit_at (pos) ; parents . push ((parent_commit . id () . into () , (parent_commit . generation () , parent_commit . committer_timestamp () as i64) ,)) ; if first_only { break ; } } } } Ok (parents) }
};
}
