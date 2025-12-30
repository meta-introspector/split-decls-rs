// Generated macro for impl_738 (impl)
macro_rules! Depcrate_open_permissionsimpl_738 {
() => {
// Module: crate::open::permissions
// Provides: {"impl_738"}
// Dependencies: {}
impl gix_sec :: trust :: DefaultForLevel for Permissions { fn default_for_level (level : Trust) -> Self { match level { Trust :: Full => Permissions :: all () , Trust :: Reduced => Permissions :: secure () , } } }
};
}
