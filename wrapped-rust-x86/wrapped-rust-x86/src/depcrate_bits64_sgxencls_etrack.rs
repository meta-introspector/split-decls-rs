// Generated macro for encls_etrack (function)
macro_rules! Depcrate_bits64_sgxencls_etrack {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_etrack"}
// Dependencies: {}
# [doc = " Activates EBLOCK Checks."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Pointer to the SECS of the EPC page."] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_etrack (secs_pointer : u64) -> u32 { encls ! (EnclsCommand :: ETRACK as u64 , secs_pointer) . 0 }
};
}
