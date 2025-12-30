// Generated macro for ProcessInstruction (type)
macro_rules! Depcrate_entrypoint_deprecatedProcessInstruction {
() => {
// Module: crate::entrypoint_deprecated
// Provides: {"ProcessInstruction"}
// Dependencies: {}
# [doc = " User implemented function to process an instruction"] # [doc = ""] # [doc = " program_id: Program ID of the currently executing program"] # [doc = " accounts: Accounts passed as part of the instruction"] # [doc = " instruction_data: Instruction data"] pub type ProcessInstruction = fn (program_id : & Pubkey , accounts : & [AccountInfo] , instruction_data : & [u8]) -> ProgramResult ;
};
}
