macro_rules! deps {
    () => {
        Error!();
        Driver!();
        IndexObjectFn!();
        Configuration!();
    };
}

macro_rules! to_git {
    () => {
        deps!();
        # [doc = ""] pub mod to_git { # [doc = " A function that fills `buf` `fn(&mut buf)` with the data stored in the index of the file that should be converted."] pub type IndexObjectFn < 'a > = dyn FnMut (& mut Vec < u8 >) -> Result < Option < () > , gix_object :: find :: Error > + 'a ; # [doc = " The error returned by [Pipeline::convert_to_git()][super::Pipeline::convert_to_git()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Eol (# [from] crate :: eol :: convert_to_git :: Error) , # [error (transparent)] Worktree (# [from] crate :: worktree :: encode_to_git :: Error) , # [error (transparent)] Driver (# [from] crate :: driver :: apply :: Error) , # [error (transparent)] Configuration (# [from] super :: configuration :: Error) , # [error ("Copy of driver process output to memory failed")] ReadProcessOutputToBuffer (# [from] std :: io :: Error) , # [error ("Could not allocate buffer")] OutOfMemory (# [from] std :: collections :: TryReserveError) , } }
    };
}

to_git!()