// Generated macro for CALLBACK_STACK_SIZE (const)
macro_rules! Depcrate_shims_native_lib_traceCALLBACK_STACK_SIZE {
() => {
// Module: crate::shims::native_lib::trace
// Provides: {"CALLBACK_STACK_SIZE"}
// Dependencies: {}
# [doc = " The size of the temporary stack we use for callbacks that the server executes in the client."] # [doc = " This should be big enough that `mempr_on` and `mempr_off` can safely be jumped into with the"] # [doc = " stack pointer pointing to a \"stack\" of this size without overflowing it."] const CALLBACK_STACK_SIZE : usize = 1024 ;
};
}
