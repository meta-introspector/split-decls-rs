// Generated macro for get_stack_height (function)
macro_rules! Depcrate_syscallsget_stack_height {
() => {
// Module: crate::syscalls
// Provides: {"get_stack_height"}
// Dependencies: {}
# [doc = " Get the current stack height."] # [doc = ""] # [doc = " Transaction-level instructions are height"] # [doc = " [`crate::TRANSACTION_LEVEL_STACK_HEIGHT`]`, fist invoked inner instruction"] # [doc = " is height `TRANSACTION_LEVEL_STACK_HEIGHT + 1`, and so forth."] # [cfg (feature = "syscalls")] pub fn get_stack_height () -> usize { # [cfg (target_os = "solana")] unsafe { sol_get_stack_height () as usize } # [cfg (not (target_os = "solana"))] 0 }
};
}
