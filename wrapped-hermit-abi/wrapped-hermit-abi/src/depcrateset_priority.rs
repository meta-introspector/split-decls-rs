// Generated macro for set_priority (function)
macro_rules! Depcrateset_priority {
() => {
// Module: crate
// Provides: {"set_priority"}
// Dependencies: {}
# [doc = " Determine the priority of the current thread"] # [inline (always)] pub unsafe fn set_priority (tid : Tid , prio : Priority) { sys_set_priority (tid , prio . into ()) ; }
};
}
