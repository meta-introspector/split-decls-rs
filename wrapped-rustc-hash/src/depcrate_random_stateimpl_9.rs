// Generated macro for impl_9 (impl)
macro_rules! Depcrate_random_stateimpl_9 {
() => {
// Module: crate::random_state
// Provides: {"impl_9"}
// Dependencies: {}
impl FxRandomState { # [doc = " Constructs a new `FxRandomState` that is initialized with random seed."] pub fn new () -> FxRandomState { use rand :: Rng ; use std :: { cell :: Cell , thread_local } ; thread_local ! (static SEED : Cell < usize > = { Cell :: new (rand :: thread_rng () . gen ()) }) ; SEED . with (| seed | { let s = seed . get () ; seed . set (s . wrapping_add (1)) ; FxRandomState { seed : s } }) } }
};
}
