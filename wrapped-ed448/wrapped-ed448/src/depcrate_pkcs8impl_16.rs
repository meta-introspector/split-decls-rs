// Generated macro for impl_16 (impl)
macro_rules! Depcrate_pkcs8impl_16 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_16"}
// Dependencies: {}
impl KeypairBytes { # [doc = " Size of an Ed448 keypair when serialized as bytes."] const BYTE_SIZE : usize = 114 ; # [doc = " Parse raw keypair from a 114-byte input."] pub fn from_bytes (bytes : & [u8 ; Self :: BYTE_SIZE]) -> Self { let (sk , pk) = bytes . split_at (Self :: BYTE_SIZE / 2) ; Self { secret_key : sk . try_into () . expect ("secret key size error") , public_key : Some (PublicKeyBytes (pk . try_into () . expect ("public key size error") ,)) , } } # [doc = " Serialize as a 114-byte keypair."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - `Some(bytes)` if the `public_key` is present."] # [doc = " - `None` if the `public_key` is absent (i.e. `None`)."] pub fn to_bytes (& self) -> Option < [u8 ; Self :: BYTE_SIZE] > { if let Some (public_key) = & self . public_key { let mut result = [0u8 ; Self :: BYTE_SIZE] ; let (sk , pk) = result . split_at_mut (Self :: BYTE_SIZE / 2) ; sk . copy_from_slice (& self . secret_key) ; pk . copy_from_slice (public_key . as_ref ()) ; Some (result) } else { None } } }
};
}
