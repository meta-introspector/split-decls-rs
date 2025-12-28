macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! is_active {
    () => {
        deps!();
        # [doc = ""] pub mod is_active { # [doc = " The error returned by [Submodule::is_active()](crate::Submodule::is_active())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] InitIsActivePlatform (# [from] gix_submodule :: is_active_platform :: Error) , # [error (transparent)] QueryIsActive (# [from] gix_config :: value :: Error) , # [error (transparent)] InitAttributes (# [from] crate :: config :: attribute_stack :: Error) , # [error (transparent)] InitPathspecDefaults (# [from] gix_pathspec :: defaults :: from_environment :: Error) , # [error (transparent)] ObtainIndex (# [from] crate :: repository :: index_or_load_from_head :: Error) , } }
    };
}

is_active!();