// Generated macro for impl_1188 (impl)
macro_rules! Depcrate_iter_try_reduce_withimpl_1188 {
() => {
// Module: crate::iter::try_reduce_with
// Provides: {"impl_1188"}
// Dependencies: {}
impl < 'r , R , T > Reducer < Option < T > > for TryReduceWithConsumer < 'r , R > where R : Fn (T :: Output , T :: Output) -> T + Sync , T : Try , { fn reduce (self , left : Option < T > , right : Option < T >) -> Option < T > { let reduce_op = self . reduce_op ; match (left , right) { (Some (left) , Some (right)) => match (left . branch () , right . branch ()) { (Continue (left) , Continue (right)) => Some (reduce_op (left , right)) , (Break (r) , _) | (_ , Break (r)) => Some (T :: from_residual (r)) , } , (None , x) | (x , None) => x , } } }
};
}
