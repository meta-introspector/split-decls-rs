macro_rules! deps {
    () => {
        SnapshotMut!();
    };
}

macro_rules! impl_560 {
    () => {
        deps!();
        impl Deref for SnapshotMut < '_ > { type Target = gix_config :: File < 'static > ; fn deref (& self) -> & Self :: Target { & self . config } }
    };
}

impl_560!();