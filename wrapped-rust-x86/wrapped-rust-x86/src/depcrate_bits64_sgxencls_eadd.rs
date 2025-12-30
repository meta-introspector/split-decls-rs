// Generated macro for encls_eadd (function)
macro_rules! Depcrate_bits64_sgxencls_eadd {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_eadd"}
// Dependencies: {}
# [doc = " Add a Page to an Uninitialized Enclave."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a PAGEINFO."] # [doc = "  * Address of the destination EPC page."] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_eadd (pageinfo : u64 , epc_page : u64) { encls ! (EnclsCommand :: EADD as u64 , pageinfo , epc_page) ; }
};
}
