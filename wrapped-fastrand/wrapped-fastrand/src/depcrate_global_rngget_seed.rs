// Generated macro for get_seed (function)
macro_rules! Depcrate_global_rngget_seed {
() => {
// Module: crate::global_rng
// Provides: {"get_seed"}
// Dependencies: {}
# [doc = " Gives back **current** seed that is being held by the thread-local generator."] # [inline] pub fn get_seed () -> u64 { with_rng (| r | r . get_seed ()) }
};
}
