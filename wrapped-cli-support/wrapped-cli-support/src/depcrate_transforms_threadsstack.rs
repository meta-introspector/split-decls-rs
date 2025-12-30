// Generated macro for Stack (struct)
macro_rules! Depcrate_transforms_threadsStack {
() => {
// Module: crate::transforms::threads
// Provides: {"Stack"}
// Dependencies: {}
struct Stack { # [doc = " The stack pointer global"] pointer : GlobalId , # [doc = " The address of a small, \"scratch-space\" stack"] temp : i32 , # [doc = " The address of a lock for the temporary stack"] temp_lock : i32 , # [doc = " A global to store allocated stack"] alloc : GlobalId , # [doc = " The size of the stack"] size : GlobalId , }
};
}
