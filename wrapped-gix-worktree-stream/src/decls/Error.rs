macro_rules! Error {
    () => {
        # [doc = " The error returned by [`next_entry()`][Stream::next_entry()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Io (# [from] std :: io :: Error) , # [error ("Could not find a tree's leaf, typically a blob")] Find (# [from] gix_object :: find :: existing :: Error) , # [error ("Could not find a tree to traverse")] FindTree (# [from] gix_object :: find :: existing_iter :: Error) , # [error ("Could not query attributes for path \"{path}\"")] Attributes { path : BString , source : Box < dyn std :: error :: Error + Send + Sync + 'static > , } , # [error (transparent)] Traverse (# [from] gix_traverse :: tree :: breadthfirst :: Error) , # [error (transparent)] ConvertToWorktree (# [from] gix_filter :: pipeline :: convert :: to_worktree :: Error) , }
    };
}

Error!()