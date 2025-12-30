// Generated macro for BoundSender (type)
macro_rules! DepcrateBoundSender {
() => {
// Module: crate
// Provides: {"BoundSender"}
// Dependencies: {}
# [cfg (any (target_os = "linux" , target_os = "android" , target_os = "windows"))] pub (crate) type BoundSender < T > = std :: sync :: mpsc :: SyncSender < T > ;
};
}
