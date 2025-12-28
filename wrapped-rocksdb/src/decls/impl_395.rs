macro_rules! deps {
    () => {
        DBAccess!();
        SnapshotWithThreadMode!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        # [doc = " `Send` and `Sync` implementations for `SnapshotWithThreadMode` are safe, because `SnapshotWithThreadMode` is"] # [doc = " immutable and can be safely shared between threads."] unsafe impl < D : DBAccess > Send for SnapshotWithThreadMode < '_ , D > { }
    };
}

impl_395!();