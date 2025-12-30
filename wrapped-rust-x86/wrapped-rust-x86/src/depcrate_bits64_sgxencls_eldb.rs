// Generated macro for encls_eldb (function)
macro_rules! Depcrate_bits64_sgxencls_eldb {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_eldb"}
// Dependencies: {}
# [doc = " Loads and verifies an EPC page and marks the page as blocked."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of the PAGEINFO"] # [doc = "  * Address of the EPC page"] # [doc = "  * Address of the version-array slot"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_eldb (pageinfo : u64 , epc_page : u64 , verion_array_slot : u64) -> u32 { encls ! (EnclsCommand :: ELDB as u64 , pageinfo , epc_page , verion_array_slot) . 0 }
};
}
