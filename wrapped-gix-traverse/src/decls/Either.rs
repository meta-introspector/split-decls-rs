macro_rules! Either {
    () => {
        # [doc = " Information about a commit that can be obtained either from a [`gix_object::CommitRefIter`] or"] # [doc = " a [`gix_commitgraph::file::Commit`]."] # [derive (Clone , Copy)] pub enum Either < 'buf , 'cache > { # [doc = " See [`gix_object::CommitRefIter`]."] CommitRefIter (gix_object :: CommitRefIter < 'buf >) , # [doc = " See [`gix_commitgraph::file::Commit`]."] CachedCommit (gix_commitgraph :: file :: Commit < 'cache >) , }
    };
}

Either!()