// Generated macro for impl_314 (impl)
macro_rules! Depcrate_dsaimpl_314 {
() => {
// Module: crate::dsa
// Provides: {"impl_314"}
// Dependencies: {}
impl Dsa < Params > { # [doc = " Creates a DSA params based upon the given parameters."] # [corresponds (DSA_set0_pqg)] pub fn from_pqg (p : BigNum , q : BigNum , g : BigNum) -> Result < Dsa < Params > , ErrorStack > { unsafe { let dsa = Dsa :: from_ptr (cvt_p (ffi :: DSA_new ()) ?) ; cvt (DSA_set0_pqg (dsa . 0 , p . as_ptr () , q . as_ptr () , g . as_ptr ())) ? ; mem :: forget ((p , q , g)) ; Ok (dsa) } } # [doc = " Generates DSA params based on the given number of bits."] # [corresponds (DSA_generate_parameters_ex)] pub fn generate_params (bits : u32) -> Result < Dsa < Params > , ErrorStack > { ffi :: init () ; unsafe { let dsa = Dsa :: from_ptr (cvt_p (ffi :: DSA_new ()) ?) ; cvt (ffi :: DSA_generate_parameters_ex (dsa . 0 , bits as BitType , ptr :: null () , 0 , ptr :: null_mut () , ptr :: null_mut () , ptr :: null_mut () ,)) ? ; Ok (dsa) } } # [doc = " Generates a private key based on the DSA params."] # [corresponds (DSA_generate_key)] pub fn generate_key (self) -> Result < Dsa < Private > , ErrorStack > { unsafe { let dsa_ptr = self . 0 ; cvt (ffi :: DSA_generate_key (dsa_ptr)) ? ; mem :: forget (self) ; Ok (Dsa :: from_ptr (dsa_ptr)) } } }
};
}
