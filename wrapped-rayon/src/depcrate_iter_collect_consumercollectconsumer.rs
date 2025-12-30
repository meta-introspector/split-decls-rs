// Generated macro for CollectConsumer (struct)
macro_rules! Depcrate_iter_collect_consumerCollectConsumer {
() => {
// Module: crate::iter::collect::consumer
// Provides: {"CollectConsumer"}
// Dependencies: {}
pub (super) struct CollectConsumer < 'c , T : Send > { # [doc = " See `CollectResult` for explanation of why this is not a slice"] start : SendPtr < T > , len : usize , marker : PhantomData < & 'c mut T > , }
};
}
