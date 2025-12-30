// Generated macro for impl_428 (impl)
macro_rules! Depcrate_iter_collect_consumerimpl_428 {
() => {
// Module: crate::iter::collect::consumer
// Provides: {"impl_428"}
// Dependencies: {}
impl < 'c , T > Drop for CollectResult < 'c , T > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (ptr :: slice_from_raw_parts_mut (self . start . 0 , self . initialized_len ,)) ; } } }
};
}
