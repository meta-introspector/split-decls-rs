// Generated macro for impl_180 (impl)
macro_rules! Depcrate_context_itemsimpl_180 {
() => {
// Module: crate::context_items
// Provides: {"impl_180"}
// Dependencies: {}
impl < M : Message > ActorDelayedMessageItem < M > { pub fn new (msg : M , timeout : Duration) -> Self { Self { msg : Some (msg) , timeout : actix_rt :: time :: sleep (timeout) , } } }
};
}
