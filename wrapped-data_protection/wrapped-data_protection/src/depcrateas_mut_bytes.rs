// Generated macro for as_mut_bytes (function)
macro_rules! Depcrateas_mut_bytes {
() => {
// Module: crate
// Provides: {"as_mut_bytes"}
// Dependencies: {}
# [expect (clippy :: mut_from_ref)] unsafe fn as_mut_bytes (buffer : & IBuffer) -> Result < & mut [u8] > { let interop = buffer . cast :: < IBufferByteAccess > () ? ; unsafe { let data = interop . Buffer () ? ; Ok (std :: slice :: from_raw_parts_mut (data , buffer . Length () ? as usize ,)) } }
};
}
