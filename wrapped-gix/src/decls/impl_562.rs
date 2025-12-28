macro_rules! deps {
    () => {
        CommitAutoRollback!();
        Repository!();
    };
}

macro_rules! impl_562 {
    () => {
        deps!();
        impl Deref for CommitAutoRollback < '_ > { type Target = crate :: Repository ; fn deref (& self) -> & Self :: Target { self . repo . as_ref () . expect ("always present") } }
    };
}

impl_562!();