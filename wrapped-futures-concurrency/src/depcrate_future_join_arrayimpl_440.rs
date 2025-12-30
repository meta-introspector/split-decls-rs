// Generated macro for impl_440 (impl)
macro_rules! Depcrate_future_join_arrayimpl_440 {
() => {
// Module: crate::future::join::array
// Provides: {"impl_440"}
// Dependencies: {}
# [doc = " Drop the already initialized values on cancellation."] # [pinned_drop] impl < Fut , const N : usize > PinnedDrop for Join < Fut , N > where Fut : Future , { fn drop (self : Pin < & mut Self >) { let mut this = self . project () ; for i in this . state . ready_indexes () { unsafe { this . items . drop (i) } ; } for i in this . state . pending_indexes () { unsafe { this . futures . as_mut () . drop (i) } ; } } }
};
}
