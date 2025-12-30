// Generated macro for impl_576 (impl)
macro_rules! Depcrate_stabilityimpl_576 {
() => {
// Module: crate::stability
// Provides: {"impl_576"}
// Dependencies: {}
impl Stability { pub fn is_unstable (& self) -> bool { self . level . is_unstable () } pub fn is_stable (& self) -> bool { self . level . is_stable () } pub fn stable_since (& self) -> Option < StableSince > { self . level . stable_since () } }
};
}
