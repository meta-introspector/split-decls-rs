// Generated macro for impl_1171 (impl)
macro_rules! Depcrate_iter_try_reduceimpl_1171 {
() => {
// Module: crate::iter::try_reduce
// Provides: {"impl_1171"}
// Dependencies: {}
impl < 'r , R , ID , T > Consumer < T > for TryReduceConsumer < 'r , R , ID > where R : Fn (T :: Output , T :: Output) -> T + Sync , ID : Fn () -> T :: Output + Sync , T : Try + Send , { type Folder = TryReduceFolder < 'r , R , T > ; type Reducer = Self ; type Result = T ; fn split_at (self , _index : usize) -> (Self , Self , Self) { (self , self , self) } fn into_folder (self) -> Self :: Folder { TryReduceFolder { reduce_op : self . reduce_op , control : Continue ((self . identity) ()) , full : self . full , } } fn full (& self) -> bool { self . full . load (Ordering :: Relaxed) } }
};
}
