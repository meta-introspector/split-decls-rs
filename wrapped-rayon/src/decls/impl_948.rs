macro_rules! deps {
    () => {
        ParallelIterator!();
        UnzipA!();
        UnzipOp!();
        ParallelExtend!();
        UnindexedConsumer!();
        UnzipB!();
    };
}

macro_rules! impl_948 {
    () => {
        deps!();
        impl < 'b , I , OP , FromB > ParallelIterator for UnzipA < 'b , I , OP , FromB > where I : ParallelIterator , OP : UnzipOp < I :: Item > , FromB : Send + ParallelExtend < OP :: Right > , { type Item = OP :: Left ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let mut result = None ; { let iter = UnzipB { base : self . base , op : self . op , left_consumer : consumer , left_result : & mut result , } ; self . b . par_extend (iter) ; } result . expect ("unzip consumers didn't execute!") } fn opt_len (& self) -> Option < usize > { if OP :: indexable () { self . base . opt_len () } else { None } } }
    };
}

impl_948!();