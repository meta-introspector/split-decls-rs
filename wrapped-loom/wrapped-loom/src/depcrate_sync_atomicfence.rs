// Generated macro for fence (function)
macro_rules! Depcrate_sync_atomicfence {
() => {
// Module: crate::sync::atomic
// Provides: {"fence"}
// Dependencies: {}
# [doc = " An atomic fence."] pub fn fence (order : Ordering) { crate :: rt :: fence (order) ; }
};
}
