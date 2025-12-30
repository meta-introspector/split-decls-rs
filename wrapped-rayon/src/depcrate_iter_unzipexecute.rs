// Generated macro for execute (function)
macro_rules! Depcrate_iter_unzipexecute {
() => {
// Module: crate::iter::unzip
// Provides: {"execute"}
// Dependencies: {}
# [doc = " Runs an unzip-like operation into default `ParallelExtend` collections."] fn execute < I , OP , FromA , FromB > (pi : I , op : OP) -> (FromA , FromB) where I : ParallelIterator , OP : UnzipOp < I :: Item > , FromA : Default + Send + ParallelExtend < OP :: Left > , FromB : Default + Send + ParallelExtend < OP :: Right > , { let mut a = FromA :: default () ; let mut b = FromB :: default () ; execute_into (& mut a , & mut b , pi , op) ; (a , b) }
};
}
