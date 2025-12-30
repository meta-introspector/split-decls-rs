// Generated macro for fill_okm (function)
macro_rules! Depcrate_hkdffill_okm {
() => {
// Module: crate::hkdf
// Provides: {"fill_okm"}
// Dependencies: {}
fn fill_okm (prk : & Prk , info : & [& [u8]] , out : & mut [u8] , len : usize ,) -> Result < () , error :: Unspecified > { if out . len () != len { return Err (error :: Unspecified) ; } let digest_alg = prk . 0 . algorithm () . digest_algorithm () ; assert ! (digest_alg . block_len () >= digest_alg . output_len ()) ; let mut ctx = hmac :: Context :: with_key (& prk . 0) ; let mut n = 1u8 ; let mut out = out ; loop { for info in info { ctx . update (info) ; } ctx . update (& [n]) ; let t = ctx . sign () ; let t = t . as_ref () ; out = if out . len () < digest_alg . output_len () { let len = out . len () ; out . copy_from_slice (& t [.. len]) ; & mut [] } else { let (this_chunk , rest) = out . split_at_mut (digest_alg . output_len ()) ; this_chunk . copy_from_slice (t) ; rest } ; if out . is_empty () { return Ok (()) ; } ctx = hmac :: Context :: with_key (& prk . 0) ; ctx . update (t) ; n = n . checked_add (1) . unwrap () ; } }
};
}
