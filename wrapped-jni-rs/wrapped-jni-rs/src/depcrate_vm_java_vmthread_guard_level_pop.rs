// Generated macro for thread_guard_level_pop (function)
macro_rules! Depcrate_vm_java_vmthread_guard_level_pop {
() => {
// Module: crate::vm::java_vm
// Provides: {"thread_guard_level_pop"}
// Dependencies: {}
# [doc = " Decrements the thread guard level, returning the new level."] fn thread_guard_level_pop () -> usize { let level = THREAD_GUARD_NEST_LEVEL . with (| cell | { let level = cell . get () ; assert_ne ! (level , 0 , "Spuriously dropped more AttachGuards than were known to exist") ; cell . set (level - 1) ; level - 1 }) ; if level == 0 { THREAD_ATTACHMENT . set (std :: ptr :: null_mut ()) ; } level }
};
}
