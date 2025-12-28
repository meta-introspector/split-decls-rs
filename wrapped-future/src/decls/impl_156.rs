macro_rules! deps {
    () => {
        SyncState!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        unsafe impl < T : Async > Send for SyncState < T > { }
    };
}

impl_156!();