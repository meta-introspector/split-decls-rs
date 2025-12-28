macro_rules! deps {
    () => {
        Convert!();
        RebaseOperationType!();
    };
}

macro_rules! impl_593 {
    () => {
        deps!();
        impl RebaseOperationType { # [doc = " Convert from the int into an enum. Returns None if invalid."] pub fn from_raw (raw : raw :: git_rebase_operation_t) -> Option < RebaseOperationType > { match raw { raw :: GIT_REBASE_OPERATION_PICK => Some (RebaseOperationType :: Pick) , raw :: GIT_REBASE_OPERATION_REWORD => Some (RebaseOperationType :: Reword) , raw :: GIT_REBASE_OPERATION_EDIT => Some (RebaseOperationType :: Edit) , raw :: GIT_REBASE_OPERATION_SQUASH => Some (RebaseOperationType :: Squash) , raw :: GIT_REBASE_OPERATION_FIXUP => Some (RebaseOperationType :: Fixup) , raw :: GIT_REBASE_OPERATION_EXEC => Some (RebaseOperationType :: Exec) , _ => None , } } }
    };
}

impl_593!();