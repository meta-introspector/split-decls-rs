macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
        Windows!();
    };
}

macro_rules! impl_1253 {
    () => {
        deps!();
        impl < 'data , T : Sync > ParallelIterator for Windows < 'data , T > { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1253!();