// Generated macro for impl_184 (impl)
macro_rules! Depcrate_future_future_sharedimpl_184 {
() => {
// Module: crate::future::future::shared
// Provides: {"impl_184"}
// Dependencies: {}
impl < Fut > Drop for Shared < Fut > where Fut : Future , { fn drop (& mut self) { if self . waker_key != NULL_WAKER_KEY { if let Some (ref inner) = self . inner { # [cfg (feature = "std")] if let Ok (mut wakers) = inner . notifier . wakers . lock () { if let Some (wakers) = wakers . as_mut () { wakers . remove (self . waker_key) ; } } # [cfg (not (feature = "std"))] if let Some (wakers) = inner . notifier . wakers . lock () . as_mut () { wakers . remove (self . waker_key) ; } } } } }
};
}
