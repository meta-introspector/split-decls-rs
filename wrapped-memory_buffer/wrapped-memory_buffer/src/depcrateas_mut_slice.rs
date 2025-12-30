// Generated macro for as_mut_slice (function)
macro_rules! Depcrateas_mut_slice {
() => {
// Module: crate
// Provides: {"as_mut_slice"}
// Dependencies: {}
# [expect (clippy :: mut_from_ref)] unsafe fn as_mut_slice (buffer : & IMemoryBufferReference) -> Result < & mut [u8] > { let interop = buffer . cast :: < IMemoryBufferByteAccess > () ? ; let mut data = std :: ptr :: null_mut () ; let mut len = 0 ; unsafe { interop . GetBuffer (& mut data , & mut len) ? ; Ok (std :: slice :: from_raw_parts_mut (data , len as usize)) } }
};
}
