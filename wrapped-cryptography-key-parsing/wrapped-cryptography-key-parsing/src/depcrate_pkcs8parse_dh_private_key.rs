// Generated macro for parse_dh_private_key (function)
macro_rules! Depcrate_pkcs8parse_dh_private_key {
() => {
// Module: crate::pkcs8
// Provides: {"parse_dh_private_key"}
// Dependencies: {}
# [cfg (not (CRYPTOGRAPHY_IS_BORINGSSL))] fn parse_dh_private_key (private_key_data : & [u8] , p : openssl :: bn :: BigNum , g : openssl :: bn :: BigNum , q : Option < openssl :: bn :: BigNum > ,) -> KeyParsingResult < openssl :: pkey :: PKey < openssl :: pkey :: Private > > { let private_key_bytes = asn1 :: parse_single :: < asn1 :: BigUint < '_ > > (private_key_data) ? . as_bytes () ; let dh_private_key = openssl :: bn :: BigNum :: from_slice (private_key_bytes) ? ; if p . num_bits () < MIN_DH_MODULUS_SIZE as i32 { return Err (KeyParsingError :: InvalidKey) ; } let dh = openssl :: dh :: Dh :: from_pqg (p , q , g) ? ; let dh = dh . set_private_key (dh_private_key) ? ; Ok (openssl :: pkey :: PKey :: from_dh (dh) ?) }
};
}
