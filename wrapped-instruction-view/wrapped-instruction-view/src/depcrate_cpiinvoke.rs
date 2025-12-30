// Generated macro for invoke (function)
macro_rules! Depcrate_cpiinvoke {
() => {
// Module: crate::cpi
// Provides: {"invoke"}
// Dependencies: {}
# [doc = " Invoke a cross-program instruction from an array of `AccountView`s."] # [doc = ""] # [doc = " This function is a convenience wrapper around the [`invoke_signed`] function"] # [doc = " with the signers' seeds set to an empty slice."] # [doc = ""] # [doc = " Note that this function is inlined to avoid the overhead of a function call,"] # [doc = " but uses stack memory allocation. When a large number of accounts is needed,"] # [doc = " it is recommended to use the [`invoke_with_slice`] function instead to reduce"] # [doc = " stack memory utilization."] # [doc = ""] # [doc = " # Important"] # [doc = ""] # [doc = " The accounts on the `account_views` slice must be in the same order as the"] # [doc = " `accounts` field of the `instruction`. When the instruction has duplicated"] # [doc = " accounts, it is necessary to pass a duplicated reference to the same account"] # [doc = " to maintain the 1:1 relationship between accounts and instruction accounts."] # [inline (always)] pub fn invoke < const ACCOUNTS : usize > (instruction : & InstructionView , account_views : & [& AccountView ; ACCOUNTS] ,) -> ProgramResult { invoke_signed :: < ACCOUNTS > (instruction , account_views , & []) }
};
}
