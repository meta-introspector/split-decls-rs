// Generated macro for impl_661 (impl)
macro_rules! Depcrate_future_try_join_arrayimpl_661 {
() => {
// Module: crate::future::try_join::array
// Provides: {"impl_661"}
// Dependencies: {}
# [doc = " Drop the already initialized values on cancellation."] # [pinned_drop] impl < Fut , T , E , const N : usize > PinnedDrop for TryJoin < Fut , T , E , N > where Fut : Future < Output = Result < T , E > > , { fn drop (self : Pin < & mut Self >) { let mut this = self . project () ; for i in this . state . ready_indexes () { unsafe { this . items . drop (i) } ; } for i in this . state . pending_indexes () { unsafe { this . futures . as_mut () . drop (i) } ; } } }
};
}
