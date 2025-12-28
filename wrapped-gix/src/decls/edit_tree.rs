macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! edit_tree {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "tree-editor")] pub mod edit_tree { # [doc = " The error returned by [Repository::edit_tree()](crate::Repository::edit_tree)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FindTree (# [from] crate :: object :: find :: existing :: with_conversion :: Error) , # [error (transparent)] InitEditor (# [from] crate :: object :: tree :: editor :: init :: Error) , } }
    };
}

edit_tree!()