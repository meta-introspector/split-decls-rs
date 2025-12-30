// Generated macro for execute_into (function)
macro_rules! Depcrate_iter_unzipexecute_into {
() => {
// Module: crate::iter::unzip
// Provides: {"execute_into"}
// Dependencies: {}
# [doc = " Runs an unzip-like operation into `ParallelExtend` collections."] fn execute_into < I , OP , FromA , FromB > (a : & mut FromA , b : & mut FromB , pi : I , op : OP) where I : ParallelIterator , OP : UnzipOp < I :: Item > , FromA : Send + ParallelExtend < OP :: Left > , FromB : Send + ParallelExtend < OP :: Right > , { let iter = UnzipA { base : pi , op , b } ; a . par_extend (iter) ; }
};
}
