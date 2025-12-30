// Generated macro for check_sync (function)
macro_rules! Depcrate_synccheck_sync {
() => {
// Module: crate::sync
// Provides: {"check_sync"}
// Dependencies: {}
# [test] fn check_sync () { fn for_sync_alloc < A : Allocator + Sync > () { fn is_sink < T : Sync > () { } is_sink :: < SyncBlinkAlloc < A > > () ; } for_sync_alloc :: < Global > () ; }
};
}
