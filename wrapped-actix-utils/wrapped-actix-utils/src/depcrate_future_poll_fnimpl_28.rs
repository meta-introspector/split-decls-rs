// Generated macro for impl_28 (impl)
macro_rules! Depcrate_future_poll_fnimpl_28 {
() => {
// Module: crate::future::poll_fn
// Provides: {"impl_28"}
// Dependencies: {}
impl < F , T > Future for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { type Output = T ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { # [allow (clippy :: needless_borrow)] (unsafe { & mut self . get_unchecked_mut () . f }) (cx) } }
};
}
