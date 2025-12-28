macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! pipeline {
    () => {
        deps!();
        # [doc = ""] pub mod pipeline { # [doc = ""] pub mod options { use crate :: { bstr :: BString , config } ; # [doc = " The error returned by [Pipeline::options()](crate::filter::Pipeline::options())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] CheckRoundTripEncodings (# [from] config :: encoding :: Error) , # [error (transparent)] SafeCrlf (# [from] config :: key :: GenericErrorWithValue) , # [error ("Could not interpret 'filter.{name}.required' configuration")] Driver { name : BString , source : gix_config :: value :: Error , } , # [error (transparent)] CommandContext (# [from] config :: command_context :: Error) , } } # [doc = ""] pub mod convert_to_git { # [doc = " The error returned by [Pipeline::convert_to_git()](crate::filter::Pipeline::convert_to_git())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to prime attributes to the path at which the data resides")] WorktreeCacheAtPath (# [from] std :: io :: Error) , # [error (transparent)] Convert (# [from] gix_filter :: pipeline :: convert :: to_git :: Error) , } } # [doc = ""] pub mod convert_to_worktree { # [doc = " The error returned by [Pipeline::convert_to_worktree()](crate::filter::Pipeline::convert_to_worktree())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to prime attributes to the path at which the data resides")] WorktreeCacheAtPath (# [from] std :: io :: Error) , # [error (transparent)] Convert (# [from] gix_filter :: pipeline :: convert :: to_worktree :: Error) , } } # [doc = ""] pub mod worktree_file_to_object { use std :: path :: PathBuf ; # [doc = " The error returned by [Pipeline::worktree_file_to_object()](crate::filter::Pipeline::worktree_file_to_object())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Cannot add worktree files in bare repositories")] MissingWorktree , # [error ("Failed to perform IO for object creation for '{}'" , path . display ())] IO { source : std :: io :: Error , path : PathBuf } , # [error (transparent)] WriteBlob (# [from] crate :: object :: write :: Error) , # [error (transparent)] ConvertToGit (# [from] crate :: filter :: pipeline :: convert_to_git :: Error) , } } }
    };
}

pipeline!()