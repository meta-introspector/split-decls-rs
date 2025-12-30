// Generated macro for ProcessInstruction (type)
macro_rules! DepcrateProcessInstruction {
() => {
// Module: crate
// Provides: {"ProcessInstruction"}
// Dependencies: {}
# [doc = " User implemented function to process an instruction"] # [doc = ""] # [doc = " program_id: Program ID of the currently executing program accounts: Accounts"] # [doc = " passed as part of the instruction instruction_data: Instruction data"] pub type ProcessInstruction = fn (program_id : & Pubkey , accounts : & [AccountInfo] , instruction_data : & [u8]) -> ProgramResult ;
};
}
