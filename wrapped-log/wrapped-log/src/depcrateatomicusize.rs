// Generated macro for AtomicUsize (struct)
macro_rules! DepcrateAtomicUsize {
() => {
// Module: crate
// Provides: {"AtomicUsize"}
// Dependencies: {}
# [cfg (not (target_has_atomic = "ptr"))] struct AtomicUsize { v : Cell < usize > , }
};
}
