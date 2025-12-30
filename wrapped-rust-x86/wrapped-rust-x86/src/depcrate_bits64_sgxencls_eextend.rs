// Generated macro for encls_eextend (function)
macro_rules! Depcrate_bits64_sgxencls_eextend {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_eextend"}
// Dependencies: {}
# [doc = " Extend Uninitialized Enclave Measurement by 256 Bytes"] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Effective address of the SECS of the data chunk"] # [doc = "  * Effective address of a 256-byte chunk in the EPC"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_eextend (secs_chunk : u64 , epc_chunk : u64) { encls ! (EnclsCommand :: EEXTEND as u64 , secs_chunk , epc_chunk) ; }
};
}
