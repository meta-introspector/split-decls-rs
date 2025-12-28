macro_rules! deps {
    () => {
        Kind!();
        Error!();
    };
}

macro_rules! worktree_stream {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "worktree-stream")] pub mod worktree_stream { # [doc = " The error returned by [`Repository::worktree_stream()`](crate::Repository::worktree_stream())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FindTree (# [from] crate :: object :: find :: existing :: Error) , # [error (transparent)] OpenTree (# [from] crate :: repository :: index_from_tree :: Error) , # [error (transparent)] AttributesCache (# [from] crate :: config :: attribute_stack :: Error) , # [error (transparent)] FilterPipeline (# [from] crate :: filter :: pipeline :: options :: Error) , # [error (transparent)] CommandContext (# [from] crate :: config :: command_context :: Error) , # [error ("Needed {id} to be a tree to turn into a workspace stream, got {actual}")] NotATree { id : gix_hash :: ObjectId , actual : gix_object :: Kind , } , } }
    };
}

worktree_stream!()