// Generated macro for macro_1839 (macro)
macro_rules! Depcrate_ntpsapimacro_1839 {
() => {
// Module: crate::ntpsapi
// Provides: {"macro_1839"}
// Dependencies: {}
# [cfg (any (target_arch = "x86_64" , target_arch = "aarch64"))] STRUCT ! { # [repr (align (16))] struct RTL_UMS_CONTEXT { Link : SINGLE_LIST_ENTRY , __padding : u64 , Context : CONTEXT , Teb : PVOID , UserContext : PVOID , ScheduledThread : ULONG , Suspended : ULONG , VolatileContext : ULONG , Terminated : ULONG , DebugActive : ULONG , RunningOnSelfThread : ULONG , DenyRunningOnSelfThread : ULONG , Flags : LONG , KernelUpdateLock : ULONG64 , PrimaryClientID : ULONG64 , ContextLock : ULONG64 , PrimaryUmsContext : * mut RTL_UMS_CONTEXT , SwitchCount : ULONG , KernelYieldCount : ULONG , MixedYieldCount : ULONG , YieldCount : ULONG , } }
};
}
