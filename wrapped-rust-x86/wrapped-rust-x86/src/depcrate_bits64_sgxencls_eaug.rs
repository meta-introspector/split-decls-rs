// Generated macro for encls_eaug (function)
macro_rules! Depcrate_bits64_sgxencls_eaug {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_eaug"}
// Dependencies: {}
# [doc = " Add a Page to an Initialized Enclave."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a SECINFO"] # [doc = "  * Address of the destination EPC page"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_eaug (secinfo_address : u64 , epc_page : u64) { encls ! (EnclsCommand :: EAUG as u64 , secinfo_address , epc_page) ; }
};
}
