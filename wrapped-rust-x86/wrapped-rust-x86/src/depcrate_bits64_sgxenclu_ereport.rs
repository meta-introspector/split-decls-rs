// Generated macro for enclu_ereport (function)
macro_rules! Depcrate_bits64_sgxenclu_ereport {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu_ereport"}
// Dependencies: {}
# [doc = " Create a Cryptographic Report of the Enclave."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of TARGETINFO"] # [doc = "  * Address of REPORTDATA"] # [doc = "  * Address where the REPORT is written to in an OUTPUTDATA"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn enclu_ereport (targetinfo : u64 , reportdata : u64 , outputdata : u64) { enclu ! (EncluCommand :: EReport as u64 , targetinfo , reportdata , outputdata) ; }
};
}
