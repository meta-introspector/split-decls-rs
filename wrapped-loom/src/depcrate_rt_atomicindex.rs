// Generated macro for index (function)
macro_rules! Depcrate_rt_atomicindex {
() => {
// Module: crate::rt::atomic
// Provides: {"index"}
// Dependencies: {}
fn index (cnt : u16) -> usize { cnt as usize % MAX_ATOMIC_HISTORY }
};
}
