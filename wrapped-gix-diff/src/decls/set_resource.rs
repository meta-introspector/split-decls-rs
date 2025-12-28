macro_rules! deps {
    () => {
        Error!();
        ResourceKind!();
    };
}

macro_rules! set_resource {
    () => {
        deps!();
        # [doc = ""] pub mod set_resource { use bstr :: BString ; use crate :: blob :: { pipeline , ResourceKind } ; # [doc = " The error returned by [Platform::set_resource](super::Platform::set_resource)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Can only diff blobs and links, not {mode:?}")] InvalidMode { mode : gix_object :: tree :: EntryKind } , # [error ("Failed to read {kind} worktree data from '{rela_path}'")] Io { rela_path : BString , kind : ResourceKind , source : std :: io :: Error , } , # [error ("Failed to obtain attributes for {kind} resource at '{rela_path}'")] Attributes { rela_path : BString , kind : ResourceKind , source : std :: io :: Error , } , # [error (transparent)] ConvertToDiffable (# [from] pipeline :: convert_to_diffable :: Error) , } }
    };
}

set_resource!();