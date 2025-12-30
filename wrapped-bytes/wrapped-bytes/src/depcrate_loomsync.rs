// Generated macro for sync (module)
macro_rules! Depcrate_loomsync {
() => {
// Module: crate::loom
// Provides: {"sync"}
// Dependencies: {}
# [cfg (all (test , loom))] pub (crate) mod sync { pub (crate) mod atomic { pub (crate) use loom :: sync :: atomic :: { AtomicPtr , AtomicUsize , Ordering } ; pub (crate) trait AtomicMut < T > { } } }
};
}
