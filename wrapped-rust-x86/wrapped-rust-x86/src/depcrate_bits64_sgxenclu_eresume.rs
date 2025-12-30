// Generated macro for enclu_eresume (function)
macro_rules! Depcrate_bits64_sgxenclu_eresume {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu_eresume"}
// Dependencies: {}
# [doc = " Re-Enters an Enclave."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a TCS."] # [doc = "  * Address of AEP."] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn enclu_eresume (tcs : u64 , aep : u64) { enclu ! (EncluCommand :: EResume as u64 , tcs , aep) ; }
};
}
