// Generated macro for prf (function)
macro_rules! Depcrate_tls_prfprf {
() => {
// Module: crate::tls_prf
// Provides: {"prf"}
// Dependencies: {}
fn prf (algorithm : & 'static Algorithm , secret : & [u8] , label : & [u8] , seed1 : & [u8] , seed2 : Option < & [u8] > , output : usize ,) -> Result < Secret , Unspecified > { if output == 0 { return Err (Unspecified) ; } let mut output = vec ! [0u8 ; output] ; let digest = match_digest_type (& algorithm . 0) ; let (seed2 , seed2_len) = if let Some (seed2) = seed2 { (seed2 . as_ptr () , seed2 . len ()) } else { (null () , 0usize) } ; if 1 != indicator_check ! (unsafe { CRYPTO_tls1_prf (* digest , output . as_mut_ptr () , output . len () , secret . as_ptr () , secret . len () , label . as_ptr () . cast () , label . len () , seed1 . as_ptr () , seed1 . len () , seed2 , seed2_len ,) }) { return Err (Unspecified) ; } Ok (Secret { algorithm , secret : output . into_boxed_slice () , }) }
};
}
