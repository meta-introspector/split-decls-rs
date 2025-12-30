// Generated macro for Queue (struct)
macro_rules! DepcrateQueue {
() => {
// Module: crate
// Provides: {"Queue"}
// Dependencies: {}
# [derive (Debug , Clone , Default , PartialEq , Eq)] struct Queue { # [doc = " Events must be stored in the following order:"] # [doc = " 1. `remove` or `move out` event"] # [doc = " 2. `rename` event"] # [doc = " 3. Other events"] events : VecDeque < DebouncedEvent > , }
};
}
