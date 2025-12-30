// Generated macro for load_instruction_at_checked (function)
macro_rules! Depcrateload_instruction_at_checked {
() => {
// Module: crate
// Provides: {"load_instruction_at_checked"}
// Dependencies: {}
# [doc = " Load an `Instruction` in the currently executing `Transaction` at the"] # [doc = " specified index."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns [`ProgramError::UnsupportedSysvar`] if the given account's ID is not equal to [`ID`]."] pub fn load_instruction_at_checked (index : usize , instruction_sysvar_account_info : & AccountInfo ,) -> Result < Instruction , ProgramError > { if ! check_id (instruction_sysvar_account_info . key) { return Err (ProgramError :: UnsupportedSysvar) ; } let instruction_sysvar = instruction_sysvar_account_info . try_borrow_data () ? ; load_instruction_at (index , & instruction_sysvar) . map_err (| err | match err { SanitizeError :: IndexOutOfBounds => ProgramError :: InvalidArgument , _ => ProgramError :: InvalidInstructionData , }) }
};
}
