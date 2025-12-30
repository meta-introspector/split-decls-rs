// Generated macro for bounded (function)
macro_rules! Depcratebounded {
() => {
// Module: crate
// Provides: {"bounded"}
// Dependencies: {}
# [cfg (any (target_os = "linux" , target_os = "android" , target_os = "windows"))] # [inline] pub (crate) fn bounded < T > (cap : usize) -> (BoundSender < T > , Receiver < T >) { std :: sync :: mpsc :: sync_channel (cap) }
};
}
