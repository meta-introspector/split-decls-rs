macro_rules! deps {
    () => {
        Handle!();
        AutoRemove!();
        ContainingDirectory!();
    };
}

macro_rules! mark_at_with_permissions {
    () => {
        deps!();
        # [doc = " Like [`mark_at`], but allows to set the given filesystem `permissions`."] pub fn mark_at_with_permissions (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove , permissions : std :: fs :: Permissions ,) -> io :: Result < Handle < Closed > > { Handle :: < Closed > :: at_with_permissions (path , directory , cleanup , permissions) }
    };
}

mark_at_with_permissions!()