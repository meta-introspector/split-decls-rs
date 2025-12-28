macro_rules! deps {
    () => {
        SharedFileSnapshotMut!();
        SharedFileSnapshot!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : std :: fmt :: Debug > Deref for SharedFileSnapshotMut < T > { type Target = MutableOnDemand < Option < SharedFileSnapshot < T > > > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_15!()