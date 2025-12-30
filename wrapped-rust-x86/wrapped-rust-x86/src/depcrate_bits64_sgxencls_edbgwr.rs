// Generated macro for encls_edbgwr (function)
macro_rules! Depcrate_bits64_sgxencls_edbgwr {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_edbgwr"}
// Dependencies: {}
# [doc = " Write to a Debug Enclave."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Data to be written to a debug enclave"] # [doc = "  * Address of Target memory in the EPC"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_edbgwr (data : u64 , target_address : u64) { encls ! (EnclsCommand :: EDBGWR as u64 , data , target_address) ; }
};
}
