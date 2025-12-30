// Generated macro for get_instruction_relative (function)
macro_rules! Depcrateget_instruction_relative {
() => {
// Module: crate
// Provides: {"get_instruction_relative"}
// Dependencies: {}
# [doc = " Returns the `Instruction` relative to the current `Instruction` in the"] # [doc = " currently executing `Transaction`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns [`ProgramError::UnsupportedSysvar`] if the given account's ID is not equal to [`ID`]."] pub fn get_instruction_relative (index_relative_to_current : i64 , instruction_sysvar_account_info : & AccountInfo ,) -> Result < Instruction , ProgramError > { if ! check_id (instruction_sysvar_account_info . key) { return Err (ProgramError :: UnsupportedSysvar) ; } let instruction_sysvar = instruction_sysvar_account_info . data . borrow () ; let current_index = load_current_index (& instruction_sysvar) as i64 ; let index = current_index . saturating_add (index_relative_to_current) ; if index < 0 { return Err (ProgramError :: InvalidArgument) ; } load_instruction_at (current_index . saturating_add (index_relative_to_current) as usize , & instruction_sysvar ,) . map_err (| err | match err { SanitizeError :: IndexOutOfBounds => ProgramError :: InvalidArgument , _ => ProgramError :: InvalidInstructionData , }) }
};
}
