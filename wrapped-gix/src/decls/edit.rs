macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! edit {
    () => {
        deps!();
        # [doc = ""] pub mod edit { use crate :: config ; # [doc = " The error returned by [`edit_references(…)`][crate::Repository::edit_references()], and others"] # [doc = " which ultimately create a reference."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FileTransactionPrepare (# [from] gix_ref :: file :: transaction :: prepare :: Error) , # [error (transparent)] FileTransactionCommit (# [from] gix_ref :: file :: transaction :: commit :: Error) , # [error (transparent)] NameValidation (# [from] gix_validate :: reference :: name :: Error) , # [error ("Could not interpret core.filesRefLockTimeout or core.packedRefsTimeout, it must be the number in milliseconds to wait for locks or negative to wait forever")] LockTimeoutConfiguration (# [from] config :: lock_timeout :: Error) , # [error (transparent)] ParseCommitterTime (# [from] crate :: config :: time :: Error) , } }
    };
}

edit!();