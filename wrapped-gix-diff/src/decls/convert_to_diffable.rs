macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! convert_to_diffable {
    () => {
        deps!();
        # [doc = ""] pub mod convert_to_diffable { use std :: collections :: TryReserveError ; use bstr :: BString ; use gix_object :: tree :: EntryKind ; # [doc = " The error returned by [Pipeline::convert_to_diffable()](super::Pipeline::convert_to_diffable())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Entry at '{rela_path}' must be regular file or symlink, but was {actual:?}")] InvalidEntryKind { rela_path : BString , actual : EntryKind } , # [error ("Entry at '{rela_path}' is declared as symlink but symlinks are disabled via core.symlinks")] SymlinkDisabled { rela_path : BString } , # [error ("Entry at '{rela_path}' could not be read as symbolic link")] ReadLink { rela_path : BString , source : std :: io :: Error } , # [error ("Entry at '{rela_path}' could not be opened for reading or read from")] OpenOrRead { rela_path : BString , source : std :: io :: Error } , # [error ("Entry at '{rela_path}' could not be copied from a filter process to a memory buffer")] StreamCopy { rela_path : BString , source : std :: io :: Error } , # [error ("Failed to run '{cmd}' for binary-to-text conversion of entry at {rela_path}")] RunTextConvFilter { rela_path : BString , cmd : String , source : std :: io :: Error , } , # [error ("Tempfile for binary-to-text conversion for entry at {rela_path} could not be created")] CreateTempfile { rela_path : BString , source : std :: io :: Error } , # [error ("Binary-to-text conversion '{cmd}' for entry at {rela_path} failed with: {stderr}")] TextConvFilterFailed { rela_path : BString , cmd : String , stderr : BString , } , # [error (transparent)] FindObject (# [from] gix_object :: find :: existing_object :: Error) , # [error (transparent)] ConvertToWorktree (# [from] gix_filter :: pipeline :: convert :: to_worktree :: Error) , # [error (transparent)] ConvertToGit (# [from] gix_filter :: pipeline :: convert :: to_git :: Error) , # [error ("Memory allocation failed")] OutOfMemory (# [from] TryReserveError) , } }
    };
}

convert_to_diffable!()