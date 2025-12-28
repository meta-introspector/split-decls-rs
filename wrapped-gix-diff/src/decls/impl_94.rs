macro_rules! deps {
    () => {
        ResourceKind!();
        WorktreeRoots!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        # [doc = " Access"] impl WorktreeRoots { # [doc = " Return the root path for the given `kind`"] pub fn by_kind (& self , kind : ResourceKind) -> Option < & Path > { match kind { ResourceKind :: OldOrSource => self . old_root . as_deref () , ResourceKind :: NewOrDestination => self . new_root . as_deref () , } } # [doc = " Return `true` if all worktree roots are unset."] pub fn is_unset (& self) -> bool { self . new_root . is_none () && self . old_root . is_none () } }
    };
}

impl_94!();