// Generated macro for impl_78 (impl)
macro_rules! Depcrate_referencesimpl_78 {
() => {
// Module: crate::references
// Provides: {"impl_78"}
// Dependencies: {}
impl ReferenceStyle { # [track_caller] fn parse (arg : & str) -> Self { match arg { "full" => Self :: Full , "flat" => Self :: Flat , "skip-root" => Self :: SkipRoot , _ => invalid_reference () , } } }
};
}
