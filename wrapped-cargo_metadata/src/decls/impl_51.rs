macro_rules! deps {
    () => {
        PackageId!();
        WorkspaceDefaultMembers!();
        Target!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl core :: ops :: Deref for WorkspaceDefaultMembers { type Target = [PackageId] ; fn deref (& self) -> & Self :: Target { self . 0 . as_ref () . expect ("WorkspaceDefaultMembers should only be dereferenced on Cargo versions >= 1.71") } }
    };
}

impl_51!()