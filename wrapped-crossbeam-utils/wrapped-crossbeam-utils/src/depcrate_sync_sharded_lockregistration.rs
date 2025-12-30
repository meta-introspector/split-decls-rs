// Generated macro for Registration (struct)
macro_rules! Depcrate_sync_sharded_lockRegistration {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"Registration"}
// Dependencies: {}
# [doc = " A registration of a thread with an index."] # [doc = ""] # [doc = " When dropped, unregisters the thread and frees the reserved index."] struct Registration { index : usize , thread_id : ThreadId , }
};
}
