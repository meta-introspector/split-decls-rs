macro_rules! deps {
    () => {
        Iter!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_1077 {
    () => {
        deps!();
        impl ParallelIterator for Iter < char > { type Item = char ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { convert_char ! (self . drive (consumer)) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1077!()