// Generated macro for TrustAnchor (struct)
macro_rules! DepcrateTrustAnchor {
() => {
// Module: crate
// Provides: {"TrustAnchor"}
// Dependencies: {}
# [doc = " A trust anchor (a.k.a. root CA)"] # [doc = ""] # [doc = " Traditionally, certificate verification libraries have represented trust anchors as full X.509"] # [doc = " root certificates. However, those certificates contain a lot more data than is needed for"] # [doc = " verifying certificates. The [`TrustAnchor`] representation allows an application to store"] # [doc = " just the essential elements of trust anchors."] # [doc = ""] # [doc = " The most common way to get one of these is to call [`rustls_webpki::anchor_from_trusted_cert()`]."] # [doc = ""] # [doc = " [`rustls_webpki::anchor_from_trusted_cert()`]: https://docs.rs/rustls-webpki/latest/webpki/fn.anchor_from_trusted_cert.html"] # [derive (Clone , Debug , Hash , PartialEq , Eq)] pub struct TrustAnchor < 'a > { # [doc = " Value of the `subject` field of the trust anchor"] pub subject : Der < 'a > , # [doc = " Value of the `subjectPublicKeyInfo` field of the trust anchor"] pub subject_public_key_info : Der < 'a > , # [doc = " Value of DER-encoded `NameConstraints`, containing name constraints to the trust anchor, if any"] pub name_constraints : Option < Der < 'a > > , }
};
}
