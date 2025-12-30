// Generated macro for impl_554 (impl)
macro_rules! Depcrate_diffimpl_554 {
() => {
// Module: crate::diff
// Provides: {"impl_554"}
// Dependencies: {}
impl < 'a > DiffBinaryFile < 'a > { # [doc = " The type of binary data for this file"] pub fn kind (& self) -> DiffBinaryKind { unsafe { Binding :: from_raw ((* self . raw) . kind) } } # [doc = " The binary data, deflated"] pub fn data (& self) -> & [u8] { unsafe { slice :: from_raw_parts ((* self . raw) . data as * const u8 , (* self . raw) . datalen as usize) } } # [doc = " The length of the binary data after inflation"] pub fn inflated_len (& self) -> usize { unsafe { (* self . raw) . inflatedlen as usize } } }
};
}
