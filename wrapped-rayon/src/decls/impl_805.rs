macro_rules! deps {
    () => {
        UnindexedConsumer!();
        RepeatN!();
        ParallelIterator!();
    };
}

macro_rules! impl_805 {
    () => {
        deps!();
        impl < T > ParallelIterator for RepeatN < T > where T : Clone + Send , { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . inner . len ()) } }
    };
}

impl_805!()