macro_rules! deps {
    () => {
        Error!();
        Either!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Either < '_ , '_ > { # [doc = " Get a commit’s `tree_id` by either getting it from a [`gix_commitgraph::Graph`], if"] # [doc = " present, or a [`gix_object::CommitRefIter`] otherwise."] pub fn tree_id (self) -> Result < ObjectId , gix_object :: decode :: Error > { match self { Self :: CommitRefIter (mut commit_ref_iter) => commit_ref_iter . tree_id () , Self :: CachedCommit (commit) => Ok (commit . root_tree_id () . into ()) , } } # [doc = " Get a committer timestamp by either getting it from a [`gix_commitgraph::Graph`], if"] # [doc = " present, or a [`gix_object::CommitRefIter`] otherwise."] pub fn commit_time (self) -> Result < gix_date :: SecondsSinceUnixEpoch , gix_object :: decode :: Error > { match self { Self :: CommitRefIter (commit_ref_iter) => commit_ref_iter . committer () . map (| c | c . seconds ()) , Self :: CachedCommit (commit) => Ok (commit . committer_timestamp () as gix_date :: SecondsSinceUnixEpoch) , } } }
    };
}

impl_34!()