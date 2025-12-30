// Generated macro for Stack (struct)
macro_rules! Depcrate_walkStack {
() => {
// Module: crate::walk
// Provides: {"Stack"}
// Dependencies: {}
# [doc = " A work-stealing stack."] # [derive (Debug)] struct Stack { # [doc = " This thread's index."] index : usize , # [doc = " The thread-local stack."] deque : Deque < Message > , # [doc = " The work stealers."] stealers : Arc < [Stealer < Message >] > , }
};
}
