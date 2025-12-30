// Generated macro for TzifTransitionsOwned (type)
macro_rules! Depcrate_sharedTzifTransitionsOwned {
() => {
// Module: crate::shared
// Provides: {"TzifTransitionsOwned"}
// Dependencies: {}
# [doc = " An alias for TZif transition data whose backing storage is on the heap."] # [cfg (feature = "alloc")] pub type TzifTransitionsOwned = TzifTransitions < alloc :: vec :: Vec < i64 > , alloc :: vec :: Vec < TzifDateTime > , alloc :: vec :: Vec < TzifDateTime > , alloc :: vec :: Vec < TzifTransitionInfo > , > ;
};
}
