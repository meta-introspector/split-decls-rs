macro_rules! deps {
    () => {
        SnapshotWithThreadMode!();
        DBAccess!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        unsafe impl < D : DBAccess > Sync for SnapshotWithThreadMode < '_ , D > { }
    };
}

impl_396!()