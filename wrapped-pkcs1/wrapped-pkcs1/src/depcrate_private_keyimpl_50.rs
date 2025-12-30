// Generated macro for impl_50 (impl)
macro_rules! Depcrate_private_keyimpl_50 {
() => {
// Module: crate::private_key
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for RsaPrivateKey < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { let version = Version :: decode (reader) ? ; let result = Self { modulus : reader . decode () ? , public_exponent : reader . decode () ? , private_exponent : reader . decode () ? , prime1 : reader . decode () ? , prime2 : reader . decode () ? , exponent1 : reader . decode () ? , exponent2 : reader . decode () ? , coefficient : reader . decode () ? , other_prime_infos : reader . decode () ? , } ; if version . is_multi () != result . other_prime_infos . is_some () { return Err (reader . error (der :: ErrorKind :: Value { tag : Tag :: Integer })) ; } Ok (result) } }
};
}
