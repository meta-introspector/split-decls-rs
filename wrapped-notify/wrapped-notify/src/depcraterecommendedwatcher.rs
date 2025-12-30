// Generated macro for RecommendedWatcher (type)
macro_rules! DepcrateRecommendedWatcher {
() => {
// Module: crate
// Provides: {"RecommendedWatcher"}
// Dependencies: {}
# [doc = " The recommended [`Watcher`] implementation for the current platform"] # [cfg (not (any (target_os = "linux" , target_os = "android" , target_os = "macos" , target_os = "windows" , target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "dragonfly" , target_os = "ios")))] pub type RecommendedWatcher = PollWatcher ;
};
}
