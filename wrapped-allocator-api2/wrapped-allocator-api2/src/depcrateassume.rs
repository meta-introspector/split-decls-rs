// Generated macro for assume (function)
macro_rules! Depcrateassume {
() => {
// Module: crate
// Provides: {"assume"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [track_caller] # [inline (always)] # [cfg (not (debug_assertions))] unsafe fn assume (v : bool) { if ! v { unsafe { core :: hint :: unreachable_unchecked () ; } } }
};
}
