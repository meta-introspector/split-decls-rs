// Generated macro for with_rng (function)
macro_rules! Depcrate_global_rngwith_rng {
() => {
// Module: crate::global_rng
// Provides: {"with_rng"}
// Dependencies: {}
# [doc = " Run an operation with the current thread-local generator."] # [inline] fn with_rng < R > (f : impl FnOnce (& mut Rng) -> R) -> R { RNG . with (| rng | { let current = rng . replace (Rng (0)) ; let mut restore = RestoreOnDrop { rng , current } ; f (& mut restore . current) }) }
};
}
