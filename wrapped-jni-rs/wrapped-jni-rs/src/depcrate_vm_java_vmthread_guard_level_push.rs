// Generated macro for thread_guard_level_push (function)
macro_rules! Depcrate_vm_java_vmthread_guard_level_push {
() => {
// Module: crate::vm::java_vm
// Provides: {"thread_guard_level_push"}
// Dependencies: {}
# [doc = " Increments the thread guard level, returning the new level."] fn thread_guard_level_push (env : * mut jni_sys :: JNIEnv) -> usize { THREAD_GUARD_NEST_LEVEL . with (| cell | { let level = cell . get () ; if level == 0 { THREAD_ATTACHMENT . set (env) ; } cell . set (level + 1) ; level + 1 }) }
};
}
