macro_rules! deps {
    () => {
        UnindexedConsumer!();
        Consumer!();
        ParallelIterator!();
        UnzipOp!();
    };
}

macro_rules! UnzipB {
    () => {
        deps!();
        # [doc = " A fake iterator to intercept the `Consumer` for type `B`."] struct UnzipB < 'r , I , OP , CA > where I : ParallelIterator , OP : UnzipOp < I :: Item > , CA : UnindexedConsumer < OP :: Left > , { base : I , op : OP , left_consumer : CA , left_result : & 'r mut Option < CA :: Result > , }
    };
}

UnzipB!()