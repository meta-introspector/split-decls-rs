macro_rules! deps {
    () => {
        ParallelIterator!();
        UnindexedConsumer!();
        Intersperse!();
        IntersperseConsumer!();
    };
}

macro_rules! impl_645 {
    () => {
        deps!();
        impl < I > ParallelIterator for Intersperse < I > where I : ParallelIterator < Item : Clone > , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < I :: Item > , { let consumer1 = IntersperseConsumer :: new (consumer , self . item) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { match self . base . opt_len () ? { 0 => Some (0) , len => len . checked_add (len - 1) , } } }
    };
}

impl_645!()