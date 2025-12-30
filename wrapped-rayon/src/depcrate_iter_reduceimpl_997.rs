// Generated macro for impl_997 (impl)
macro_rules! Depcrate_iter_reduceimpl_997 {
() => {
// Module: crate::iter::reduce
// Provides: {"impl_997"}
// Dependencies: {}
impl < 'r , R , ID , T > Reducer < T > for ReduceConsumer < 'r , R , ID > where R : Fn (T , T) -> T + Sync , { fn reduce (self , left : T , right : T) -> T { (self . reduce_op) (left , right) } }
};
}
