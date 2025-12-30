// Generated macro for sol_log_params (function)
macro_rules! Depcrate_logsol_log_params {
() => {
// Module: crate::log
// Provides: {"sol_log_params"}
// Dependencies: {}
# [doc = " Print the hexadecimal representation of the program's input parameters."] # [doc = ""] # [doc = " - `accounts` - A slice of [`AccountInfo`]."] # [doc = " - `data` - The instruction data."] pub fn sol_log_params (accounts : & [AccountInfo] , data : & [u8]) { for (i , account) in accounts . iter () . enumerate () { msg ! ("AccountInfo") ; sol_log_64 (0 , 0 , 0 , 0 , i as u64) ; msg ! ("- Is signer") ; sol_log_64 (0 , 0 , 0 , 0 , account . is_signer as u64) ; msg ! ("- Key") ; account . key . log () ; msg ! ("- Lamports") ; sol_log_64 (0 , 0 , 0 , 0 , account . lamports ()) ; msg ! ("- Account data length") ; sol_log_64 (0 , 0 , 0 , 0 , account . data_len () as u64) ; msg ! ("- Owner") ; account . owner . log () ; } msg ! ("Instruction data") ; sol_log_slice (data) ; }
};
}
