// Generated macro for enclu_eexit (function)
macro_rules! Depcrate_bits64_sgxenclu_eexit {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu_eexit"}
// Dependencies: {}
# [doc = " Exits an Enclave."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Target address outside the enclave"] # [doc = "  * Address of the current AEP"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn enclu_eexit (ip : u64 , aep : u64) { enclu ! (EncluCommand :: EExit as u64 , ip , aep) ; }
};
}
