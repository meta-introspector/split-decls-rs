// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl Signature { # [doc = " Size of an encoded Ed25519 signature in bytes."] pub const BYTE_SIZE : usize = COMPONENT_SIZE * 2 ; # [doc = " Parse an Ed25519 signature from a byte slice."] pub fn from_bytes (bytes : & SignatureBytes) -> Self { let mut R = ComponentBytes :: default () ; let mut s = ComponentBytes :: default () ; let components = bytes . split_at (COMPONENT_SIZE) ; R . copy_from_slice (components . 0) ; s . copy_from_slice (components . 1) ; Self { R , s } } # [doc = " Parse an Ed25519 signature from its `R` and `s` components."] pub fn from_components (R : ComponentBytes , s : ComponentBytes) -> Self { Self { R , s } } # [doc = " Parse an Ed25519 signature from a byte slice."] # [doc = ""] # [doc = " # Returns"] # [doc = " - `Ok` on success"] # [doc = " - `Err` if the input byte slice is not 64-bytes"] pub fn from_slice (bytes : & [u8]) -> signature :: Result < Self > { SignatureBytes :: try_from (bytes) . map (Into :: into) . map_err (| _ | Error :: new ()) } # [doc = " Bytes for the `R` component of a signature."] pub fn r_bytes (& self) -> & ComponentBytes { & self . R } # [doc = " Bytes for the `s` component of a signature."] pub fn s_bytes (& self) -> & ComponentBytes { & self . s } # [doc = " Return the inner byte array."] pub fn to_bytes (& self) -> SignatureBytes { let mut ret = [0u8 ; Self :: BYTE_SIZE] ; let (R , s) = ret . split_at_mut (COMPONENT_SIZE) ; R . copy_from_slice (& self . R) ; s . copy_from_slice (& self . s) ; ret } # [doc = " Convert this signature into a byte vector."] # [cfg (feature = "alloc")] pub fn to_vec (& self) -> Vec < u8 > { self . to_bytes () . to_vec () } }
};
}
