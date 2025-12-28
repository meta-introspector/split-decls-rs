macro_rules! deps {
    () => {
        SnapshotMut!();
    };
}

macro_rules! impl_558 {
    () => {
        deps!();
        impl Drop for SnapshotMut < '_ > { fn drop (& mut self) { if let Some (repo) = self . repo . take () { self . commit_inner (repo) . ok () ; } } }
    };
}

impl_558!()