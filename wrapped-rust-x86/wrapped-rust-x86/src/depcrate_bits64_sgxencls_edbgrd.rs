// Generated macro for encls_edbgrd (function)
macro_rules! Depcrate_bits64_sgxencls_edbgrd {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_edbgrd"}
// Dependencies: {}
# [doc = " Read From a Debug Enclave."] # [doc = ""] # [doc = " # Return"] # [doc = " Data read from a debug enclave."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of source memory in the EPC"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_edbgrd (source_address : u64) -> u64 { encls ! (EnclsCommand :: EDBGRD as u64 , source_address) . 1 }
};
}
