// Generated macro for encls_einit (function)
macro_rules! Depcrate_bits64_sgxencls_einit {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_einit"}
// Dependencies: {}
# [doc = " Initialize an Enclave for Execution"] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of SIGSTRUCT"] # [doc = "  * Address of SECS"] # [doc = "  * Address of EINITTOKEN"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_einit (sigstruct : u64 , secs : u64 , einittoken : u64) -> u32 { encls ! (EnclsCommand :: EINIT as u64 , sigstruct , secs , einittoken) . 0 }
};
}
