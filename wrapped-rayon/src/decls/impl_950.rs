macro_rules! deps {
    () => {
        ParallelIterator!();
        UnzipOp!();
        UnzipB!();
        UnindexedConsumer!();
        UnzipConsumer!();
    };
}

macro_rules! impl_950 {
    () => {
        deps!();
        impl < 'r , I , OP , CA > ParallelIterator for UnzipB < 'r , I , OP , CA > where I : ParallelIterator , OP : UnzipOp < I :: Item > , CA : UnindexedConsumer < OP :: Left > , { type Item = OP :: Right ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = UnzipConsumer { op : & self . op , left : self . left_consumer , right : consumer , } ; let result = self . base . drive_unindexed (consumer) ; * self . left_result = Some (result . 0) ; result . 1 } fn opt_len (& self) -> Option < usize > { if OP :: indexable () { self . base . opt_len () } else { None } } }
    };
}

impl_950!()