// Generated macro for timer_after (function)
macro_rules! Depcrate_utilstimer_after {
() => {
// Module: crate::utils
// Provides: {"timer_after"}
// Dependencies: {}
# [cfg (any (feature = "unstable" , feature = "default"))] pub (crate) fn timer_after (dur : std :: time :: Duration) -> timer :: Timer { Timer :: after (dur) }
};
}
