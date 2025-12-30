// Generated macro for encls_ewb (function)
macro_rules! Depcrate_bits64_sgxencls_ewb {
() => {
// Module: crate::bits64::sgx
// Provides: {"encls_ewb"}
// Dependencies: {}
# [doc = " Invalidate an EPC Page and Write out to Main Memory."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of the EPC page."] # [doc = "  * Address of a VA slot."] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn encls_ewb (pageinfo : u64 , epc_page : u64 , va_slot : u64) -> u32 { encls ! (EnclsCommand :: EWB as u64 , pageinfo , epc_page , va_slot) . 0 }
};
}
