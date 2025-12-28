macro_rules! deps {
    () => {
        SnapshotMut!();
    };
}

macro_rules! impl_564 {
    () => {
        deps!();
        impl DerefMut for SnapshotMut < '_ > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . config } }
    };
}

impl_564!()