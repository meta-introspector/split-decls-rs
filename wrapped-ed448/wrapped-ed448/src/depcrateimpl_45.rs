// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl Signature { # [doc = " Size of an encoded Ed448 signature in bytes."] pub const BYTE_SIZE : usize = COMPONENT_SIZE * 2 ; # [doc = " Parse an Ed448 signature from a byte slice."] pub fn from_bytes (bytes : & SignatureBytes) -> Self { let mut R = [0 ; COMPONENT_SIZE] ; let mut s = [0 ; COMPONENT_SIZE] ; let components = bytes . split_at (COMPONENT_SIZE) ; R . copy_from_slice (components . 0) ; s . copy_from_slice (components . 1) ; Self { R , s } } # [doc = " Parse an Ed448 signature from a byte slice."] # [doc = ""] # [doc = " # Returns"] # [doc = " - `Ok` on success"] # [doc = " - `Err` if the input byte slice is not 64-bytes"] pub fn from_slice (bytes : & [u8]) -> signature :: Result < Self > { SignatureBytes :: try_from (bytes) . map (Into :: into) . map_err (| _ | Error :: new ()) } # [doc = " Bytes for the `R` component of a signature."] pub fn r_bytes (& self) -> & ComponentBytes { & self . R } # [doc = " Bytes for the `s` component of a signature."] pub fn s_bytes (& self) -> & ComponentBytes { & self . s } # [doc = " Return the inner byte array."] pub fn to_bytes (& self) -> SignatureBytes { let mut ret = [0u8 ; Self :: BYTE_SIZE] ; let (R , s) = ret . split_at_mut (COMPONENT_SIZE) ; R . copy_from_slice (& self . R) ; s . copy_from_slice (& self . s) ; ret } # [doc = " Create a [`Signature`] from the serialized `r` and `s` component values"] # [doc = " which comprise the signature."] pub fn from_components (r : impl Into < ComponentBytes > , s : impl Into < ComponentBytes >) -> Self { Self { R : r . into () , s : s . into () , } } }
};
}
