// Generated macro for encls_emodpr (function)
macro_rules! Depcrate_bits64_sgxencls_emodpr {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_emodpr"}
// Dependencies: {}
# [doc = " Restrict the Permissions of an EPC Page."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a SECINFO"] # [doc = "  * Address of the destination EPC page"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_emodpr (secinfo : u64 , epc_page : u64) -> u32 { encls ! (EnclsCommand :: EMODPR as u64 , secinfo , epc_page) . 0 }
};
}
