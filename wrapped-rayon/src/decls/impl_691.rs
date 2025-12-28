macro_rules! deps {
    () => {
        MapWith!();
        MapWithConsumer!();
        ParallelIterator!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_691 {
    () => {
        deps!();
        impl < I , T , F , R > ParallelIterator for MapWith < I , T , F > where I : ParallelIterator , T : Send + Clone , F : Fn (& mut T , I :: Item) -> R + Sync + Send , R : Send , { type Item = R ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = MapWithConsumer :: new (consumer , self . item , & self . map_op) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
    };
}

impl_691!()