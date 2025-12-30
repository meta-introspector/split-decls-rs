// Generated macro for SysStack (struct)
macro_rules! Depcrate_stackSysStack {
() => {
// Module: crate::stack
// Provides: {"SysStack"}
// Dependencies: {}
# [doc = " Represents any kind of stack memory."] # [doc = ""] # [doc = " `FixedSizeStack` as well as `ProtectedFixedSizeStack`"] # [doc = " can be used to allocate actual stack space."] # [derive (Debug)] pub struct SysStack { top : * mut c_void , bottom : * mut c_void , }
};
}
