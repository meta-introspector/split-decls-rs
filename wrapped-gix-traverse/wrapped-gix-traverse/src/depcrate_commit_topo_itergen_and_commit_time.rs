// Generated macro for gen_and_commit_time (function)
macro_rules! Depcrate_commit_topo_itergen_and_commit_time {
() => {
// Module: crate::commit::topo::iter
// Provides: {"gen_and_commit_time"}
// Dependencies: {}
pub (super) fn gen_and_commit_time (c : Either < '_ , '_ >) -> Result < GenAndCommitTime , Error > { match c { Either :: CommitRefIter (c) => { let mut commit_time = 0 ; for token in c { use gix_object :: commit :: ref_iter :: Token as T ; match token { Ok (T :: Tree { .. }) => continue , Ok (T :: Parent { .. }) => continue , Ok (T :: Author { .. }) => continue , Ok (T :: Committer { signature }) => { commit_time = signature . seconds () ; break ; } Ok (_unused_token) => break , Err (err) => return Err (err . into ()) , } } Ok ((gix_commitgraph :: GENERATION_NUMBER_INFINITY , commit_time)) } Either :: CachedCommit (c) => Ok ((c . generation () , c . committer_timestamp () as i64)) , } }
};
}
