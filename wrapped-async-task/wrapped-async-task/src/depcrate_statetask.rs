// Generated macro for TASK (const)
macro_rules! Depcrate_stateTASK {
() => {
// Module: crate::state
// Provides: {"TASK"}
// Dependencies: {}
# [doc = " Set if the `Task` still exists."] # [doc = ""] # [doc = " The `Task` is a special case in that it is only tracked by this flag, while all other"] # [doc = " task references (`Runnable` and `Waker`s) are tracked by the reference count."] pub (crate) const TASK : usize = 1 << 4 ;
};
}
