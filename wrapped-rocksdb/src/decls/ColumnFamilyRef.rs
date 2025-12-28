macro_rules! deps {
    () => {
        BoundColumnFamily!();
    };
}

macro_rules! ColumnFamilyRef {
    () => {
        deps!();
        # [cfg (feature = "multi-threaded-cf")] pub type ColumnFamilyRef < 'a > = Arc < BoundColumnFamily < 'a > > ;
    };
}

ColumnFamilyRef!();