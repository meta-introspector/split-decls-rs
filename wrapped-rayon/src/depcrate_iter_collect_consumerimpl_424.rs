// Generated macro for impl_424 (impl)
macro_rules! Depcrate_iter_collect_consumerimpl_424 {
() => {
// Module: crate::iter::collect::consumer
// Provides: {"impl_424"}
// Dependencies: {}
impl < 'c , T : Send + 'c > CollectConsumer < 'c , T > { # [doc = " The target memory is considered uninitialized, and will be"] # [doc = " overwritten without reading or dropping existing values."] unsafe fn new (start : * mut T , len : usize) -> Self { CollectConsumer { start : SendPtr (start) , len , marker : PhantomData , } } }
};
}
