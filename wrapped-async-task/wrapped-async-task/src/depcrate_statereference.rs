// Generated macro for REFERENCE (const)
macro_rules! Depcrate_stateREFERENCE {
() => {
// Module: crate::state
// Provides: {"REFERENCE"}
// Dependencies: {}
# [doc = " A single reference."] # [doc = ""] # [doc = " The lower bits in the state contain various flags representing the task state, while the upper"] # [doc = " bits contain the reference count. The value of `REFERENCE` represents a single reference in the"] # [doc = " total reference count."] # [doc = ""] # [doc = " Note that the reference counter only tracks the `Runnable` and `Waker`s. The `Task` is"] # [doc = " tracked separately by the `TASK` flag."] pub (crate) const REFERENCE : usize = 1 << 8 ;
};
}
