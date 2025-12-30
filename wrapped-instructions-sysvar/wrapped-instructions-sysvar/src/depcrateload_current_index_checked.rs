// Generated macro for load_current_index_checked (function)
macro_rules! Depcrateload_current_index_checked {
() => {
// Module: crate
// Provides: {"load_current_index_checked"}
// Dependencies: {}
# [doc = " Load the current `Instruction`'s index in the currently executing"] # [doc = " `Transaction`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns [`ProgramError::UnsupportedSysvar`] if the given account's ID is not equal to [`ID`]."] pub fn load_current_index_checked (instruction_sysvar_account_info : & AccountInfo ,) -> Result < u16 , ProgramError > { if ! check_id (instruction_sysvar_account_info . key) { return Err (ProgramError :: UnsupportedSysvar) ; } let instruction_sysvar = instruction_sysvar_account_info . try_borrow_data () ? ; let index = load_current_index (& instruction_sysvar) ; Ok (index) }
};
}
