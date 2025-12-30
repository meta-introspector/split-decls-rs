// Generated macro for impl_51 (impl)
macro_rules! Depcrate_schnorrimpl_51 {
() => {
// Module: crate::schnorr
// Provides: {"impl_51"}
// Dependencies: {}
impl Signature { # [doc = " Size of a Taproot Schnorr signature in bytes."] pub const BYTE_SIZE : usize = 64 ; # [doc = " Serialize this signature as bytes."] pub fn to_bytes (& self) -> SignatureBytes { let mut ret = [0 ; Self :: BYTE_SIZE] ; let (r_bytes , s_bytes) = ret . split_at_mut (Self :: BYTE_SIZE / 2) ; r_bytes . copy_from_slice (& self . r . to_bytes ()) ; s_bytes . copy_from_slice (& self . s . to_bytes ()) ; ret } # [doc = " Get the `r` component of this signature."] fn r (& self) -> & FieldElement { & self . r } # [doc = " Get the `s` component of this signature."] fn s (& self) -> & NonZeroScalar { & self . s } # [doc = " Split this signature into its `r` and `s` components."] fn split (& self) -> (& FieldElement , & NonZeroScalar) { (self . r () , self . s ()) } # [doc = " Parse a Secp256k1 signature from a byte array."] pub fn from_bytes (bytes : & SignatureBytes) -> Result < Self > { let components = FieldBytes :: slice_as_chunks (bytes) . 0 ; let r_bytes = components [0] ; let s_bytes = components [1] ; let r = FieldElement :: from_bytes (& r_bytes) . into_option () . ok_or_else (Error :: new) ? ; if r . is_zero () . into () { return Err (Error :: new ()) ; } let s = NonZeroScalar :: try_from (s_bytes . as_slice ()) . map_err (| _ | Error :: new ()) ? ; Ok (Self { r , s }) } # [doc = " Parse a Secp256k1 signature from a byte slice."] pub fn from_slice (bytes : & [u8]) -> Result < Self > { SignatureBytes :: try_from (bytes) . map_err (| _ | Error :: new ()) ? . try_into () } }
};
}
