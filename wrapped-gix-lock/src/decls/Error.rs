macro_rules! deps {
    () => {
        Marker!();
        File!();
        Fail!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The error returned when acquiring a [`File`] or [`Marker`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Another IO error occurred while obtaining the lock")] Io (# [from] std :: io :: Error) , # [error ("The lock for resource '{resource_path}' could not be obtained {mode} after {attempts} attempt(s). The lockfile at '{resource_path}{}' might need manual deletion." , super :: DOT_LOCK_SUFFIX)] PermanentlyLocked { resource_path : PathBuf , mode : Fail , attempts : usize , } , }
    };
}

Error!();