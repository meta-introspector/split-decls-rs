// Generated macro for encls_emodt (function)
macro_rules! Depcrate_bits64_sgxencls_emodt {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_emodt"}
// Dependencies: {}
# [doc = " Change the Type of an EPC Page."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a SECINFO"] # [doc = "  * Address of the destination EPC page"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_emodt (secinfo : u64 , epc_page : u64) -> u32 { encls ! (EnclsCommand :: EMODT as u64 , secinfo , epc_page) . 0 }
};
}
