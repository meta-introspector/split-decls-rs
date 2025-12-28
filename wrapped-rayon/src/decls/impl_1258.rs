macro_rules! deps {
    () => {
        ParallelIterator!();
        UnindexedConsumer!();
        IterMut!();
    };
}

macro_rules! impl_1258 {
    () => {
        deps!();
        impl < 'data , T : Send > ParallelIterator for IterMut < 'data , T > { type Item = & 'data mut T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1258!();