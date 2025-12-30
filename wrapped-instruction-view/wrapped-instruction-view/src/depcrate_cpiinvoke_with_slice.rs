// Generated macro for invoke_with_slice (function)
macro_rules! Depcrate_cpiinvoke_with_slice {
() => {
// Module: crate::cpi
// Provides: {"invoke_with_slice"}
// Dependencies: {}
# [cfg (feature = "slice-cpi")] # [doc = " Invoke a cross-program instruction from a slice of `AccountView`s."] # [doc = ""] # [doc = " This function is a convenience wrapper around the [`invoke_signed_with_slice`]"] # [doc = " function with the signers' seeds set to an empty slice."] # [doc = ""] # [doc = " Note that this function will allocate heap memory to store up to"] # [doc = " `MAX_CPI_ACCOUNTS` accounts."] # [doc = ""] # [doc = " # Important"] # [doc = ""] # [doc = " The accounts on the `account_views` slice must be in the same order as the"] # [doc = " `accounts` field of the `instruction`. When the instruction has duplicated"] # [doc = " accounts, it is necessary to pass a duplicated reference to the same account"] # [doc = " to maintain the 1:1 relationship between accounts and instruction accounts."] # [inline (always)] pub fn invoke_with_slice (instruction : & InstructionView , account_views : & [& AccountView] ,) -> ProgramResult { invoke_signed_with_slice (instruction , account_views , & []) }
};
}
