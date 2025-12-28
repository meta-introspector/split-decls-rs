macro_rules! deps {
    () => {
        WorktreeRoots!();
        ResourceKind!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl WorktreeRoots { # [doc = " Return the root path for the given `kind`"] pub fn by_kind (& self , kind : ResourceKind) -> Option < & Path > { match kind { ResourceKind :: CurrentOrOurs => self . current_root . as_deref () , ResourceKind :: CommonAncestorOrBase => self . common_ancestor_root . as_deref () , ResourceKind :: OtherOrTheirs => self . other_root . as_deref () , } } # [doc = " Return `true` if all worktree roots are unset."] pub fn is_unset (& self) -> bool { self . current_root . is_none () && self . other_root . is_none () && self . common_ancestor_root . is_none () } }
    };
}

impl_46!()