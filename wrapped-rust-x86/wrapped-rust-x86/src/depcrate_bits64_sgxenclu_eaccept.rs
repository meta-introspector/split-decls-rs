// Generated macro for enclu_eaccept (function)
macro_rules! Depcrate_bits64_sgxenclu_eaccept {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu_eaccept"}
// Dependencies: {}
# [doc = " Accept Changes to an EPC Page."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a SECINFO."] # [doc = "  * Address of the destination EPC page."] # [doc = ""] # [doc = " Returns an error code."] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn enclu_eaccept (secinfo : u64 , epc_page : u64) -> u32 { enclu ! (EncluCommand :: EAccept as u64 , secinfo , epc_page) . 0 }
};
}
