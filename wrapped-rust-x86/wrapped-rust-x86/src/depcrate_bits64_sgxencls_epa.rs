// Generated macro for encls_epa (function)
macro_rules! Depcrate_bits64_sgxencls_epa {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_epa"}
// Dependencies: {}
# [doc = " Add Version Array."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * PT_VA Constant"] # [doc = "  * Effective address of the EPC page"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_epa (pt_va : u64 , epc_page : u64) { encls ! (EnclsCommand :: EPA as u64 , pt_va , epc_page) ; }
};
}
