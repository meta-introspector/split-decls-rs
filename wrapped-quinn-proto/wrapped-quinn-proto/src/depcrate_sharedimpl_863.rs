// Generated macro for impl_863 (impl)
macro_rules! Depcrate_sharedimpl_863 {
() => {
// Module: crate::shared
// Provides: {"impl_863"}
// Dependencies: {}
impl EndpointEvent { # [doc = " Construct an event that indicating that a `Connection` will no longer emit events"] # [doc = ""] # [doc = " Useful for notifying an `Endpoint` that a `Connection` has been destroyed outside of the"] # [doc = " usual state machine flow, e.g. when being dropped by the user."] pub fn drained () -> Self { Self (EndpointEventInner :: Drained) } # [doc = " Determine whether this is the last event a `Connection` will emit"] # [doc = ""] # [doc = " Useful for determining when connection-related event loop state can be freed."] pub fn is_drained (& self) -> bool { self . 0 == EndpointEventInner :: Drained } }
};
}
