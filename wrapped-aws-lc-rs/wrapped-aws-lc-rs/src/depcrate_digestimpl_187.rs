// Generated macro for impl_187 (impl)
macro_rules! Depcrate_digestimpl_187 {
() => {
// Module: crate::digest
// Provides: {"impl_187"}
// Dependencies: {}
impl Digest { # [doc = " Imports a digest value provide by an external source. This allows for the signing of"] # [doc = " content that might not be directly accessible."] # [doc = ""] # [doc = " WARNING: Ensure that the digest is provided by a trusted source."] # [doc = " When possible, prefer to directly compute the digest of content."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns `Unspecified` if the imported value is the wrong length for the specified algorithm."] pub fn import_less_safe (digest : & [u8] , algorithm : & 'static Algorithm ,) -> Result < Self , Unspecified > { if digest . len () != algorithm . output_len { return Err (Unspecified) ; } let mut my_digest = [0u8 ; MAX_OUTPUT_LEN] ; my_digest [0 .. digest . len ()] . copy_from_slice (& digest [0 .. digest . len ()]) ; Ok (Digest { message : my_digest , len : digest . len () , algorithm , }) } # [doc = " The algorithm that was used to calculate the digest value."] # [inline] # [must_use] pub fn algorithm (& self) -> & 'static Algorithm { self . algorithm } }
};
}
