// Generated macro for encls_eremove (function)
macro_rules! Depcrate_bits64_sgxencls_eremove {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_eremove"}
// Dependencies: {}
# [doc = " Remove a page from the EPC."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Effective address of the EPC page"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_eremove (epc_page : u64) { encls ! (EnclsCommand :: EREMOVE as u64 , epc_page) ; }
};
}
