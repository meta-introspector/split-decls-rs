macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! exclude_stack {
    () => {
        deps!();
        # [doc = ""] pub mod exclude_stack { use crate :: config ; use std :: path :: PathBuf ; # [doc = " The error produced when setting up a stack to query `gitignore` information."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not read repository exclude")] Io (# [from] std :: io :: Error) , # [error (transparent)] EnvironmentPermission (# [from] gix_sec :: permission :: Error < PathBuf >) , # [error ("The value for `core.excludesFile` could not be read from configuration")] ExcludesFilePathInterpolation (# [from] gix_config :: path :: interpolate :: Error) , # [error (transparent)] ParsePreciousEnabled (# [from] config :: boolean :: Error) , } }
    };
}

exclude_stack!()