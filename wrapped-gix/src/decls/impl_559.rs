macro_rules! deps {
    () => {
        CommitAutoRollback!();
    };
}

macro_rules! impl_559 {
    () => {
        deps!();
        impl Drop for CommitAutoRollback < '_ > { fn drop (& mut self) { if let Some (repo) = self . repo . take () { self . rollback_inner (repo) . ok () ; } } }
    };
}

impl_559!();