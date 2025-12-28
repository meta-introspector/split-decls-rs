macro_rules! deps {
    () => {
        ParallelIterator!();
        UnindexedConsumer!();
        Iter!();
    };
}

macro_rules! impl_1090 {
    () => {
        deps!();
        impl < T : RangeInteger > ParallelIterator for Iter < T > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < T > , { T :: drive_unindexed (self , consumer) } # [inline] fn opt_len (& self) -> Option < usize > { T :: opt_len (self) } }
    };
}

impl_1090!();