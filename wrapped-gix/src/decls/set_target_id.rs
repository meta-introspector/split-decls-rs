macro_rules! deps {
    () => {
        Reference!();
        Error!();
        Note!();
        Object!();
    };
}

macro_rules! set_target_id {
    () => {
        deps!();
        # [doc = ""] pub mod set_target_id { use gix_ref :: { transaction :: PreviousValue , Target } ; use crate :: { bstr :: BString , Reference } ; mod error { use gix_ref :: FullName ; # [doc = " The error returned by [`Reference::set_target_id()`][super::Reference::set_target_id()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Cannot change symbolic reference {name:?} into a direct one by setting it to an id")] SymbolicReference { name : FullName } , # [error (transparent)] ReferenceEdit (# [from] crate :: reference :: edit :: Error) , } } pub use error :: Error ; impl Reference < '_ > { # [doc = " Set the id of this direct reference to `id` and use `reflog_message` for the reflog (if enabled in the repository)."] # [doc = ""] # [doc = " Note that the operation will fail on symbolic references, to change their type use the lower level reference database,"] # [doc = " or if the reference was deleted or changed in the mean time."] # [doc = " Furthermore, refrain from using this method for more than a one-off change as it creates a transaction for each invocation."] # [doc = " If multiple reference should be changed, use [`Repository::edit_references()`][crate::Repository::edit_references()]"] # [doc = " or the lower level reference database instead."] # [allow (clippy :: result_large_err)] pub fn set_target_id (& mut self , id : impl Into < gix_hash :: ObjectId > , reflog_message : impl Into < BString > ,) -> Result < () , Error > { match & self . inner . target { Target :: Symbolic (name) => return Err (Error :: SymbolicReference { name : name . clone () }) , Target :: Object (current_id) => { let changed = self . repo . reference (self . name () , id , PreviousValue :: MustExistAndMatch (Target :: Object (current_id . to_owned ())) , reflog_message ,) ? ; * self = changed ; } } Ok (()) } } }
    };
}

set_target_id!();