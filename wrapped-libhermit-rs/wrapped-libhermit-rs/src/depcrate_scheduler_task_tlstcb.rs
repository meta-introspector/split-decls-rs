// Generated macro for Tcb (struct)
macro_rules! Depcrate_scheduler_task_tlsTcb {
() => {
// Module: crate::scheduler::task::tls
// Provides: {"Tcb"}
// Dependencies: {}
# [doc = " Thread control block."] # [repr (C)] struct Tcb { # [doc = " Thread pointer."] # [cfg (target_arch = "x86_64")] thread_ptr : * mut () , # [doc = " Pointer to the dynamic thread vector (dtv)."] # [doc = ""] # [doc = " Currently not needed on Hermit."] dtv : * mut () , # [doc = " Implementation-defined TCB data."] # [doc = ""] # [doc = " Currently not used on Hermit."] tcb_data : * mut () , }
};
}
