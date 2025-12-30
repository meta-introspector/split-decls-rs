// Generated macro for impl_431 (impl)
macro_rules! Depcrate_iter_collect_consumerimpl_431 {
() => {
// Module: crate::iter::collect::consumer
// Provides: {"impl_431"}
// Dependencies: {}
# [doc = " Pretend to be unindexed for `special_collect_into_vec`,"] # [doc = " but we should never actually get used that way..."] impl < 'c , T : Send + 'c > UnindexedConsumer < T > for CollectConsumer < 'c , T > { fn split_off_left (& self) -> Self { unreachable ! ("CollectConsumer must be indexed!") } fn to_reducer (& self) -> Self :: Reducer { CollectReducer } }
};
}
