macro_rules! deps {
    () => {
        ContainingDirectory!();
        Writable!();
        AutoRemove!();
        Handle!();
    };
}

macro_rules! writable_at_with_permissions {
    () => {
        deps!();
        # [doc = " Like [`writable_at`], but allows to set the given filesystem `permissions`."] pub fn writable_at_with_permissions (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove , permissions : std :: fs :: Permissions ,) -> io :: Result < Handle < Writable > > { Handle :: < Writable > :: at_with_permissions (path , directory , cleanup , permissions) }
    };
}

writable_at_with_permissions!()