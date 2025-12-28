macro_rules! deps {
    () => {
        SnapshotWithThreadMode!();
        DB!();
    };
}

macro_rules! Snapshot {
    () => {
        deps!();
        # [doc = " A type alias to keep compatibility. See [`SnapshotWithThreadMode`] for details"] pub type Snapshot < 'a > = SnapshotWithThreadMode < 'a , DB > ;
    };
}

Snapshot!()