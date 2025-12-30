// Generated macro for impl_489 (impl)
macro_rules! Depcrate_future_join_vecimpl_489 {
() => {
// Module: crate::future::join::vec
// Provides: {"impl_489"}
// Dependencies: {}
# [doc = " Drop the already initialized values on cancellation."] # [pinned_drop] impl < Fut > PinnedDrop for Join < Fut > where Fut : Future , { fn drop (self : Pin < & mut Self >) { let mut this = self . project () ; for i in this . state . ready_indexes () { unsafe { this . items . drop (i) } ; } for i in this . state . pending_indexes () { unsafe { this . futures . as_mut () . drop (i) } ; } } }
};
}
