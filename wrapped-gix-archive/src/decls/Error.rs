macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The error returned by [`write_stream()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Io (# [from] std :: io :: Error) , # [error (transparent)] NextStreamEntry (# [from] gix_worktree_stream :: entry :: Error) , # [error ("The internal format cannot be used as an archive, it's merely a debugging tool")] InternalFormatMustNotPersist , # [error ("Support for the format '{wanted:?}' was not compiled in")] SupportNotCompiledIn { wanted : Format } , # [error ("Cannot create a zip archive if output stream does not support seek")] ZipWithoutSeek , # [error ("Cannot use modification as it is not within the supported bounds")] InvalidModificationTime (# [source] Box < dyn std :: error :: Error + Send + Sync + 'static >) , }
    };
}

Error!();