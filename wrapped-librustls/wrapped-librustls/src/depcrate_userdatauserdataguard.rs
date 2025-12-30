// Generated macro for UserdataGuard (struct)
macro_rules! Depcrate_userdataUserdataGuard {
() => {
// Module: crate::userdata
// Provides: {"UserdataGuard"}
// Dependencies: {}
# [doc = " UserdataGuard pops an entry off the USERDATA stack, restoring the"] # [doc = " thread-local state to its value previous to the creation of the UserdataGuard."] # [doc = ""] # [doc = " Invariants: As long as a UserdataGuard is live:"] # [doc = ""] # [doc = "  - The stack of userdata items for this thread must have at least one item."] # [doc = "  - The top item on that stack must be the one this guard was built with."] # [doc = "  - The `data` field must not be None."] # [doc = ""] # [doc = " If any of these invariants fails, try_drop will return an error."] pub (crate) struct UserdataGuard { data : Option < Userdata > , }
};
}
