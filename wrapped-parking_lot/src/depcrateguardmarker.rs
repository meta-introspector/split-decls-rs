// Generated macro for GuardMarker (type)
macro_rules! DepcrateGuardMarker {
() => {
// Module: crate
// Provides: {"GuardMarker"}
// Dependencies: {}
# [cfg (not (feature = "send_guard"))] type GuardMarker = lock_api :: GuardNoSend ;
};
}
