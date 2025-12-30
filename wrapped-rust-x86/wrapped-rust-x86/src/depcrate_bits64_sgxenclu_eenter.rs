// Generated macro for enclu_eenter (function)
macro_rules! Depcrate_bits64_sgxenclu_eenter {
() => {
// Module: crate::bits64::sgx
// Provides: {"enclu_eenter"}
// Dependencies: {}
# [doc = " Enters an Enclave."] # [doc = ""] # [doc = " # Arguments"] # [doc = "  * Address of a TCS."] # [doc = "  * Address of AEP."] # [doc = "  * Address of IP following EEnter."] # [doc = ""] # [doc = " Returns content of RBX.CSSA and Address of IP following EEnter."] # [doc = ""] # [doc = " # Safety"] # [doc = " Requires SGX support."] pub unsafe fn enclu_eenter (tcs : u64 , aep : u64) -> (u32 , u64) { enclu ! (EncluCommand :: EEnter as u64 , tcs , aep) }
};
}
