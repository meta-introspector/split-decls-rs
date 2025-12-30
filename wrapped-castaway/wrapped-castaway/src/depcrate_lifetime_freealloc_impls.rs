// Generated macro for alloc_impls (module)
macro_rules! Depcrate_lifetime_freealloc_impls {
() => {
// Module: crate::lifetime_free
// Provides: {"alloc_impls"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_impls { use super :: LifetimeFree ; unsafe impl LifetimeFree for alloc :: string :: String { } unsafe impl < T : LifetimeFree > LifetimeFree for alloc :: boxed :: Box < T > { } unsafe impl < T : LifetimeFree > LifetimeFree for alloc :: vec :: Vec < T > { } # [rustversion :: attr (since (1.60) , cfg (target_has_atomic = "ptr"))] unsafe impl < T : LifetimeFree > LifetimeFree for alloc :: sync :: Arc < T > { } }
};
}
