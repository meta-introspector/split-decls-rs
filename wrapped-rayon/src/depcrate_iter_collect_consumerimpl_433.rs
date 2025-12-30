// Generated macro for impl_433 (impl)
macro_rules! Depcrate_iter_collect_consumerimpl_433 {
() => {
// Module: crate::iter::collect::consumer
// Provides: {"impl_433"}
// Dependencies: {}
impl < 'c , T > Reducer < CollectResult < 'c , T > > for CollectReducer { fn reduce (self , mut left : CollectResult < 'c , T > , right : CollectResult < 'c , T > ,) -> CollectResult < 'c , T > { unsafe { let left_end = left . start . 0 . add (left . initialized_len) ; if left_end == right . start . 0 { left . total_len += right . total_len ; left . initialized_len += right . release_ownership () ; } left } } }
};
}
