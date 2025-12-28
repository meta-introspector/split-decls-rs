macro_rules! deps {
    () => {
        Worktree!();
        Error!();
        Index!();
    };
}

macro_rules! open_index {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "index")] pub mod open_index { # [doc = " The error returned by [`Worktree::open_index()`][crate::Worktree::open_index()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ConfigIndexThreads (# [from] crate :: config :: key :: GenericErrorWithValue) , # [error (transparent)] ConfigSkipHash (# [from] crate :: config :: boolean :: Error) , # [error (transparent)] IndexFile (# [from] gix_index :: file :: init :: Error) , # [error (transparent)] IndexCorrupt (# [from] gix_index :: file :: verify :: Error) , } impl crate :: Worktree < '_ > { # [doc = " A shortcut to [`crate::Repository::open_index()`]."] pub fn open_index (& self) -> Result < gix_index :: File , Error > { self . parent . open_index () } # [doc = " A shortcut to [`crate::Repository::index()`]."] pub fn index (& self) -> Result < crate :: worktree :: Index , Error > { self . parent . index () } } }
    };
}

open_index!()