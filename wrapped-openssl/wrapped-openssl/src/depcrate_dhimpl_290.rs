// Generated macro for impl_290 (impl)
macro_rules! Depcrate_dhimpl_290 {
() => {
// Module: crate::dh
// Provides: {"impl_290"}
// Dependencies: {}
impl < T > DhRef < T > where T : HasPrivate , { # [doc = " Computes a shared secret from the own private key and the given `public_key`."] # [corresponds (DH_compute_key)] pub fn compute_key (& self , public_key : & BigNumRef) -> Result < Vec < u8 > , ErrorStack > { unsafe { let key_len = ffi :: DH_size (self . as_ptr ()) ; let mut key = vec ! [0u8 ; key_len as usize] ; cvt (ffi :: DH_compute_key (key . as_mut_ptr () , public_key . as_ptr () , self . as_ptr () ,)) ? ; Ok (key) } } # [doc = " Returns the private key from the DH instance."] # [corresponds (DH_get0_key)] pub fn private_key (& self) -> & BigNumRef { let mut priv_key = ptr :: null () ; unsafe { DH_get0_key (self . as_ptr () , ptr :: null_mut () , & mut priv_key) ; BigNumRef :: from_ptr (priv_key as * mut _) } } }
};
}
