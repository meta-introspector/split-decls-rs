macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! head_id {
    () => {
        deps!();
        # [doc = ""] pub mod head_id { # [doc = " The error returned by [Submodule::head_id()](crate::Submodule::head_id())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] HeadCommit (# [from] crate :: reference :: head_commit :: Error) , # [error ("Could not get tree of head commit")] CommitTree (# [from] crate :: object :: commit :: Error) , # [error ("Could not peel tree to submodule path")] PeelTree (# [from] crate :: object :: find :: existing :: Error) , # [error (transparent)] PathConfiguration (# [from] gix_submodule :: config :: path :: Error) , } }
    };
}

head_id!()