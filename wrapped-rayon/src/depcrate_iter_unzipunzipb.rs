// Generated macro for UnzipB (struct)
macro_rules! Depcrate_iter_unzipUnzipB {
() => {
// Module: crate::iter::unzip
// Provides: {"UnzipB"}
// Dependencies: {}
# [doc = " A fake iterator to intercept the `Consumer` for type `B`."] struct UnzipB < 'r , I , OP , CA > where I : ParallelIterator , OP : UnzipOp < I :: Item > , CA : UnindexedConsumer < OP :: Left > , { base : I , op : OP , left_consumer : CA , left_result : & 'r mut Option < CA :: Result > , }
};
}
