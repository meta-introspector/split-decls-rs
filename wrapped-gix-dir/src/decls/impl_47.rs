macro_rules! deps {
    () => {
        Options!();
        Status!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Options < '_ > { fn should_hold (& self , status : entry :: Status) -> bool { if status . is_pruned () { return false ; } self . emit_ignored == Some (CollapseDirectory) || self . emit_untracked == CollapseDirectory } }
    };
}

impl_47!()