// Generated macro for impl_316 (impl)
macro_rules! Depcrate_dsaimpl_316 {
() => {
// Module: crate::dsa
// Provides: {"impl_316"}
// Dependencies: {}
impl Dsa < Public > { from_pem ! { # [doc = " Decodes a PEM-encoded SubjectPublicKeyInfo structure containing a DSA key."] # [doc = ""] # [doc = " The input should have a header of `-----BEGIN PUBLIC KEY-----`."] # [corresponds (PEM_read_bio_DSA_PUBKEY)] public_key_from_pem , Dsa < Public >, ffi :: PEM_read_bio_DSA_PUBKEY } from_der ! { # [doc = " Decodes a DER-encoded SubjectPublicKeyInfo structure containing a DSA key."] # [corresponds (d2i_DSA_PUBKEY)] public_key_from_der , Dsa < Public >, ffi :: d2i_DSA_PUBKEY } # [doc = " Create a new DSA key with only public components."] # [doc = ""] # [doc = " `p`, `q` and `g` are the common parameters."] # [doc = " `pub_key` is the public component of the key."] pub fn from_public_components (p : BigNum , q : BigNum , g : BigNum , pub_key : BigNum ,) -> Result < Dsa < Public > , ErrorStack > { ffi :: init () ; unsafe { let dsa = Dsa :: from_ptr (cvt_p (ffi :: DSA_new ()) ?) ; cvt (DSA_set0_pqg (dsa . 0 , p . as_ptr () , q . as_ptr () , g . as_ptr ())) ? ; mem :: forget ((p , q , g)) ; cvt (DSA_set0_key (dsa . 0 , pub_key . as_ptr () , ptr :: null_mut ())) ? ; mem :: forget (pub_key) ; Ok (dsa) } } }
};
}
