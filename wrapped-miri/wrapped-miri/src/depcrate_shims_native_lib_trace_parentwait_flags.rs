// Generated macro for WAIT_FLAGS (const)
macro_rules! Depcrate_shims_native_lib_trace_parentWAIT_FLAGS {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"WAIT_FLAGS"}
// Dependencies: {}
# [doc = " The flags to use when calling `waitid()`."] const WAIT_FLAGS : wait :: WaitPidFlag = wait :: WaitPidFlag :: WUNTRACED . union (wait :: WaitPidFlag :: WEXITED) ;
};
}
