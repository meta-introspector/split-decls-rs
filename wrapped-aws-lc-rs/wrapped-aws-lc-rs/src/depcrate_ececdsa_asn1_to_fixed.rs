// Generated macro for ecdsa_asn1_to_fixed (function)
macro_rules! Depcrate_ececdsa_asn1_to_fixed {
() => {
// Module: crate::ec
// Provides: {"ecdsa_asn1_to_fixed"}
// Dependencies: {}
# [inline] fn ecdsa_asn1_to_fixed (alg_id : & 'static AlgorithmID , sig : & [u8]) -> Result < Signature , Unspecified > { let expected_number_size = alg_id . private_key_size () ; let ecdsa_sig = LcPtr :: new (unsafe { ECDSA_SIG_from_bytes (sig . as_ptr () , sig . len ()) }) ? ; let r_bn = ecdsa_sig . project_const_lifetime (unsafe { | ecdsa_sig | ECDSA_SIG_get0_r (* ecdsa_sig . as_const ()) }) ? ; let r_buffer = r_bn . to_be_bytes () ; let s_bn = ecdsa_sig . project_const_lifetime (unsafe { | ecdsa_sig | ECDSA_SIG_get0_s (* ecdsa_sig . as_const ()) }) ? ; let s_buffer = s_bn . to_be_bytes () ; Ok (Signature :: new (| slice | { let (r_start , r_end) = (expected_number_size - r_buffer . len () , expected_number_size) ; let (s_start , s_end) = (2 * expected_number_size - s_buffer . len () , 2 * expected_number_size ,) ; slice [r_start .. r_end] . copy_from_slice (r_buffer . as_slice ()) ; slice [s_start .. s_end] . copy_from_slice (s_buffer . as_slice ()) ; 2 * expected_number_size })) }
};
}
