// Generated macro for encls_eblock (function)
macro_rules! Depcrate_bits64_sgxencls_eblock {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_eblock"}
// Dependencies: {}
# [doc = " Mark a page in EPC as Blocked."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Effective address of the EPC page"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_eblock (epc_page : u64) -> u32 { encls ! (EnclsCommand :: EBLOCK as u64 , epc_page) . 0 }
};
}
