// Generated macro for enclu_emodepe (function)
macro_rules! Depcrate_bits64_sgxenclu_emodepe {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu_emodepe"}
// Dependencies: {}
# [doc = " Extend an EPC Page Permissions."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a SECINFO"] # [doc = "  * Address of the destination EPC page"] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn enclu_emodepe (secinfo : u64 , epc_page : u64) { enclu ! (EncluCommand :: EModePE as u64 , secinfo , epc_page) ; }
};
}
