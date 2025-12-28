macro_rules! deps {
    () => {
        SnapshotWithThreadMode!();
        DBAccess!();
    };
}

macro_rules! impl_394 {
    () => {
        deps!();
        impl < D : DBAccess > Drop for SnapshotWithThreadMode < '_ , D > { fn drop (& mut self) { unsafe { self . db . release_snapshot (self . inner) ; } } }
    };
}

impl_394!();