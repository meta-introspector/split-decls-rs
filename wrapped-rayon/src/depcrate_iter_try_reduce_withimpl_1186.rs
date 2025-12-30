// Generated macro for impl_1186 (impl)
macro_rules! Depcrate_iter_try_reduce_withimpl_1186 {
() => {
// Module: crate::iter::try_reduce_with
// Provides: {"impl_1186"}
// Dependencies: {}
impl < 'r , R , T > Consumer < T > for TryReduceWithConsumer < 'r , R > where R : Fn (T :: Output , T :: Output) -> T + Sync , T : Try + Send , { type Folder = TryReduceWithFolder < 'r , R , T > ; type Reducer = Self ; type Result = Option < T > ; fn split_at (self , _index : usize) -> (Self , Self , Self) { (self , self , self) } fn into_folder (self) -> Self :: Folder { TryReduceWithFolder { reduce_op : self . reduce_op , opt_control : None , full : self . full , } } fn full (& self) -> bool { self . full . load (Ordering :: Relaxed) } }
};
}
