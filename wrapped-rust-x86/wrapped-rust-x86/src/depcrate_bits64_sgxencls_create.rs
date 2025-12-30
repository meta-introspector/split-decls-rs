// Generated macro for encls_create (function)
macro_rules! Depcrate_bits64_sgxencls_create {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_create"}
// Dependencies: {}
# [doc = " Create an SECS page in the Enclave Page Cache"] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a PAGEINFO"] # [doc = "  * Address of the destination SECS page"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_create (pageinfo : u64 , secs_page : u64) { encls ! (EnclsCommand :: ECREATE as u64 , pageinfo , secs_page) ; }
};
}
