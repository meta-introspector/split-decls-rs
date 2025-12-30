// Generated macro for LocationStack (struct)
macro_rules! Depcrate_memoryLocationStack {
() => {
// Module: crate::memory
// Provides: {"LocationStack"}
// Dependencies: {}
# [doc = " A History Stack."] # [derive (Debug)] struct LocationStack { prev : Vec < Location > , next : VecDeque < Location > , current : Location , }
};
}
