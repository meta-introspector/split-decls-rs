// Generated macro for encls_eldu (function)
macro_rules! Depcrate_bits64_sgxencls_eldu {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_eldu"}
// Dependencies: {}
# [doc = " Loads, verifies an EPC page and marks the page as unblocked."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of the PAGEINFO"] # [doc = "  * Address of the EPC page"] # [doc = "  * Address of the version-array slot"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_eldu (pageinfo : u64 , epc_page : u64 , verion_array_slot : u64) -> u32 { encls ! (EnclsCommand :: ELDU as u64 , pageinfo , epc_page , verion_array_slot) . 0 }
};
}
