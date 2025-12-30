// Generated macro for get_priority (function)
macro_rules! Depcrateget_priority {
() => {
// Module: crate
// Provides: {"get_priority"}
// Dependencies: {}
# [doc = " Determine the priority of the current thread"] # [inline (always)] pub unsafe fn get_priority () -> Priority { Priority :: from (sys_get_priority ()) }
};
}
