macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! from_git_dir {
    () => {
        deps!();
        # [doc = ""] pub mod from_git_dir { use crate :: file :: init ; # [doc = " The error returned by [`File::from_git_dir()`][crate::File::from_git_dir()]."] # [derive (Debug , thiserror :: Error)] pub enum Error { # [error (transparent)] FromPaths (# [from] init :: from_paths :: Error) , # [error (transparent)] FromEnv (# [from] init :: from_env :: Error) , # [error (transparent)] Init (# [from] init :: Error) , # [error (transparent)] Includes (# [from] init :: includes :: Error) , } }
    };
}

from_git_dir!();