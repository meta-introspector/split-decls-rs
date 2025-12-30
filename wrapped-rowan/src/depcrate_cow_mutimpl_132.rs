// Generated macro for impl_132 (impl)
macro_rules! Depcrate_cow_mutimpl_132 {
() => {
// Module: crate::cow_mut
// Provides: {"impl_132"}
// Dependencies: {}
impl < T : Default > Default for CowMut < '_ , T > { fn default () -> Self { CowMut :: Owned (T :: default ()) } }
};
}
