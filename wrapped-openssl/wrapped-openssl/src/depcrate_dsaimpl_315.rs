// Generated macro for impl_315 (impl)
macro_rules! Depcrate_dsaimpl_315 {
() => {
// Module: crate::dsa
// Provides: {"impl_315"}
// Dependencies: {}
impl Dsa < Private > { # [doc = " Generate a DSA key pair."] # [doc = ""] # [doc = " The `bits` parameter corresponds to the length of the prime `p`."] pub fn generate (bits : u32) -> Result < Dsa < Private > , ErrorStack > { let params = Dsa :: generate_params (bits) ? ; params . generate_key () } # [doc = " Create a DSA key pair with the given parameters"] # [doc = ""] # [doc = " `p`, `q` and `g` are the common parameters."] # [doc = " `priv_key` is the private component of the key pair."] # [doc = " `pub_key` is the public component of the key. Can be computed via `g^(priv_key) mod p`"] pub fn from_private_components (p : BigNum , q : BigNum , g : BigNum , priv_key : BigNum , pub_key : BigNum ,) -> Result < Dsa < Private > , ErrorStack > { ffi :: init () ; unsafe { let dsa = Dsa :: from_ptr (cvt_p (ffi :: DSA_new ()) ?) ; cvt (DSA_set0_pqg (dsa . 0 , p . as_ptr () , q . as_ptr () , g . as_ptr ())) ? ; mem :: forget ((p , q , g)) ; cvt (DSA_set0_key (dsa . 0 , pub_key . as_ptr () , priv_key . as_ptr ())) ? ; mem :: forget ((pub_key , priv_key)) ; Ok (dsa) } } }
};
}
