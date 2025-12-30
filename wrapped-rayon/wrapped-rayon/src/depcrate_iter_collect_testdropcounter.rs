// Generated macro for DropCounter (struct)
macro_rules! Depcrate_iter_collect_testDropCounter {
() => {
// Module: crate::iter::collect::test
// Provides: {"DropCounter"}
// Dependencies: {}
# [doc = " This counter can create elements, and then count and verify"] # [doc = " the number of which have actually been dropped again."] # [derive (Default)] struct DropCounter { created : AtomicUsize , dropped : AtomicUsize , }
};
}
