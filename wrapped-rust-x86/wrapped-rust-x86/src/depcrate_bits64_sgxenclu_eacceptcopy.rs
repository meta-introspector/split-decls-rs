// Generated macro for enclu_eacceptcopy (function)
macro_rules! Depcrate_bits64_sgxenclu_eacceptcopy {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu_eacceptcopy"}
// Dependencies: {}
# [doc = " Initialize a Pending Page."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a SECINFO."] # [doc = "  * Address of the destination EPC page."] # [doc = "  * Address of the source EPC page."] # [doc = ""] # [doc = " Returns an error code."] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn enclu_eacceptcopy (secinfo : u64 , destination_epc_page : u64 , source_epc_page : u64 ,) -> u32 { enclu ! (EncluCommand :: EAcceptCopy as u64 , secinfo , destination_epc_page , source_epc_page) . 0 }
};
}
