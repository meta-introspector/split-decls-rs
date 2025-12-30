// Generated macro for enclu_egetkey (function)
macro_rules! Depcrate_bits64_sgxenclu_egetkey {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu_egetkey"}
// Dependencies: {}
# [doc = " Retrieves a Cryptographic Key."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address to a KEYREQUEST"] # [doc = "  * Address of the OUTPUTDATA"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn enclu_egetkey (keyrequest : u64 , outputdata : u64) { enclu ! (EncluCommand :: EGetKey as u64 , keyrequest , outputdata) ; }
};
}
