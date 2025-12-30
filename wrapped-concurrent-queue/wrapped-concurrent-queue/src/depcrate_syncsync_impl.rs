// Generated macro for sync_impl (module)
macro_rules! Depcrate_syncsync_impl {
() => {
// Module: crate::sync
// Provides: {"sync_impl"}
// Dependencies: {}
# [cfg (loom)] mod sync_impl { pub (crate) use loom :: cell ; pub (crate) mod atomic { pub (crate) use loom :: sync :: atomic :: * ; } # [cfg (not (feature = "std"))] pub (crate) use loom :: hint :: spin_loop ; # [cfg (feature = "std")] pub (crate) use loom :: thread :: yield_now ; }
};
}
