// Generated macro for get_processed_sibling_instruction (function)
macro_rules! Depcrate_instructionget_processed_sibling_instruction {
() => {
// Module: crate::instruction
// Provides: {"get_processed_sibling_instruction"}
// Dependencies: {}
# [doc = " Returns a sibling instruction from the processed sibling instruction list."] # [doc = ""] # [doc = " The processed sibling instruction list is a reverse-ordered list of"] # [doc = " successfully processed sibling instructions. For example, given the call flow:"] # [doc = ""] # [doc = " A"] # [doc = " B -> C -> D"] # [doc = " B -> E"] # [doc = " B -> F"] # [doc = ""] # [doc = " Then B's processed sibling instruction list is: `[A]`"] # [doc = " Then F's processed sibling instruction list is: `[E, C]`"] pub fn get_processed_sibling_instruction (index : usize) -> Option < Instruction > { # [cfg (target_os = "solana")] { solana_instruction :: syscalls :: get_processed_sibling_instruction (index) } # [cfg (not (target_os = "solana"))] { crate :: program_stubs :: sol_get_processed_sibling_instruction (index) } }
};
}
