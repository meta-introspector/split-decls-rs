// Generated macro for impl_289 (impl)
macro_rules! Depcrate_dhimpl_289 {
() => {
// Module: crate::dh
// Provides: {"impl_289"}
// Dependencies: {}
impl < T > DhRef < T > where T : HasPublic , { # [doc = " Returns the public key from the DH instance."] # [corresponds (DH_get0_key)] pub fn public_key (& self) -> & BigNumRef { let mut pub_key = ptr :: null () ; unsafe { DH_get0_key (self . as_ptr () , & mut pub_key , ptr :: null_mut ()) ; BigNumRef :: from_ptr (pub_key as * mut _) } } }
};
}
