macro_rules! deps {
    () => {
        CommitAutoRollback!();
    };
}

macro_rules! impl_563 {
    () => {
        deps!();
        impl DerefMut for CommitAutoRollback < '_ > { fn deref_mut (& mut self) -> & mut Self :: Target { self . repo . as_mut () . expect ("always present") } }
    };
}

impl_563!();