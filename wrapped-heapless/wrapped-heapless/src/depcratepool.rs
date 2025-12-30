// Generated macro for pool (module)
macro_rules! Depcratepool {
() => {
// Module: crate
// Provides: {"pool"}
// Dependencies: {}
# [cfg (any (arm_llsc , all (target_pointer_width = "32" , any (target_has_atomic = "64" , feature = "portable-atomic")) , all (target_pointer_width = "64" , any (all (target_has_atomic = "128" , feature = "nightly") , feature = "portable-atomic"))))] pub mod pool ;
};
}
