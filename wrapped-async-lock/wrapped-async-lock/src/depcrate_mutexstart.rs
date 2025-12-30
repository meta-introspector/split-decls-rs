// Generated macro for Start (struct)
macro_rules! Depcrate_mutexStart {
() => {
// Module: crate::mutex
// Provides: {"Start"}
// Dependencies: {}
# [doc = " `pin_project_lite` doesn't support `#[cfg]` yet, so we have to do this manually."] struct Start { # [cfg (all (feature = "std" , not (target_family = "wasm")))] start : Option < Instant > , }
};
}
