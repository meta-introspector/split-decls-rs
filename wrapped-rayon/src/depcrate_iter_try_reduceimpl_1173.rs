// Generated macro for impl_1173 (impl)
macro_rules! Depcrate_iter_try_reduceimpl_1173 {
() => {
// Module: crate::iter::try_reduce
// Provides: {"impl_1173"}
// Dependencies: {}
impl < 'r , R , ID , T > Reducer < T > for TryReduceConsumer < 'r , R , ID > where R : Fn (T :: Output , T :: Output) -> T + Sync , T : Try , { fn reduce (self , left : T , right : T) -> T { match (left . branch () , right . branch ()) { (Continue (left) , Continue (right)) => (self . reduce_op) (left , right) , (Break (r) , _) | (_ , Break (r)) => T :: from_residual (r) , } } }
};
}
