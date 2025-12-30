// Generated macro for impl_995 (impl)
macro_rules! Depcrate_iter_reduceimpl_995 {
() => {
// Module: crate::iter::reduce
// Provides: {"impl_995"}
// Dependencies: {}
impl < 'r , R , ID , T > Consumer < T > for ReduceConsumer < 'r , R , ID > where R : Fn (T , T) -> T + Sync , ID : Fn () -> T + Sync , T : Send , { type Folder = ReduceFolder < 'r , R , T > ; type Reducer = Self ; type Result = T ; fn split_at (self , _index : usize) -> (Self , Self , Self) { (self , self , self) } fn into_folder (self) -> Self :: Folder { ReduceFolder { reduce_op : self . reduce_op , item : (self . identity) () , } } fn full (& self) -> bool { false } }
};
}
