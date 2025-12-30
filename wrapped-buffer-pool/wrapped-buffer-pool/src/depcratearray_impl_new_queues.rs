// Generated macro for array_impl_new_queues (macro)
macro_rules! Depcratearray_impl_new_queues {
() => {
// Module: crate
// Provides: {"array_impl_new_queues"}
// Dependencies: {}
macro_rules ! array_impl_new_queues { { $ n : expr , $ t : ident $ ($ ts : ident) * } => { impl <$ t : Default + Reuse > Pool < { $ n } , $ t > { # [allow (dead_code)] pub const fn new (limit : usize , trim : usize) -> Self { let limit = limit / $ n ; Pool { queues : [QueueShard :: new (trim , limit) , $ (QueueShard ::<$ ts >:: new (trim , limit)) ,*] , next_shard : AtomicUsize :: new (0) , } } } array_impl_new_queues ! { ($ n - 1) , $ ($ ts) * } } ; { $ n : expr , } => { } ; }
};
}
