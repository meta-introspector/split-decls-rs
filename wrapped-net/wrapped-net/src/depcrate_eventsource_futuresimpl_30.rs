// Generated macro for impl_30 (impl)
macro_rules! Depcrate_eventsource_futuresimpl_30 {
() => {
// Module: crate::eventsource::futures
// Provides: {"impl_30"}
// Dependencies: {}
# [pinned_drop] impl PinnedDrop for EventSourceSubscription { fn drop (self : Pin < & mut Self >) { let _ = self . es . remove_event_listener_with_callback ("error" , self . error_callback . as_ref () . unchecked_ref () ,) ; let _ = self . es . remove_event_listener_with_callback (& self . event_type , self . message_callback . as_ref () . unchecked_ref () ,) ; } }
};
}
