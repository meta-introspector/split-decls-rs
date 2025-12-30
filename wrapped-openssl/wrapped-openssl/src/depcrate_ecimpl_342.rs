// Generated macro for impl_342 (impl)
macro_rules! Depcrate_ecimpl_342 {
() => {
// Module: crate::ec
// Provides: {"impl_342"}
// Dependencies: {}
impl Asn1Flag { # [doc = " Curve defined using polynomial parameters"] # [doc = ""] # [doc = " Most applications use a named EC_GROUP curve, however, support"] # [doc = " is included to explicitly define the curve used to calculate keys"] # [doc = " This information would need to be known by both endpoint to make communication"] # [doc = " effective."] # [doc = ""] # [doc = " OPENSSL_EC_EXPLICIT_CURVE, but that was only added in 1.1."] # [doc = " Man page documents that 0 can be used in older versions."] # [doc = ""] # [doc = " OpenSSL documentation at [`EC_GROUP`]"] # [doc = ""] # [doc = " [`EC_GROUP`]: https://docs.openssl.org/master/man3/EC_GROUP_get_seed_len/"] pub const EXPLICIT_CURVE : Asn1Flag = Asn1Flag (0) ; # [doc = " Standard Curves"] # [doc = ""] # [doc = " Curves that make up the typical encryption use cases.  The collection of curves"] # [doc = " are well known but extensible."] # [doc = ""] # [doc = " OpenSSL documentation at [`EC_GROUP`]"] # [doc = ""] # [doc = " [`EC_GROUP`]: https://docs.openssl.org/master/man3/EC_GROUP_order_bits/"] pub const NAMED_CURVE : Asn1Flag = Asn1Flag (ffi :: OPENSSL_EC_NAMED_CURVE) ; }
};
}
