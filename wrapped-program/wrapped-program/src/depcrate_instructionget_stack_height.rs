// Generated macro for get_stack_height (function)
macro_rules! Depcrate_instructionget_stack_height {
() => {
// Module: crate::instruction
// Provides: {"get_stack_height"}
// Dependencies: {}
# [doc = " Get the current stack height, transaction-level instructions are height"] # [doc = " TRANSACTION_LEVEL_STACK_HEIGHT, fist invoked inner instruction is height"] # [doc = " TRANSACTION_LEVEL_STACK_HEIGHT + 1, etc..."] pub fn get_stack_height () -> usize { # [cfg (target_os = "solana")] { solana_instruction :: syscalls :: get_stack_height () } # [cfg (not (target_os = "solana"))] { crate :: program_stubs :: sol_get_stack_height () as usize } }
};
}
