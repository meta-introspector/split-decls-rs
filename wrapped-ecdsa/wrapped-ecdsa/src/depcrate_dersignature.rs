// Generated macro for Signature (struct)
macro_rules! Depcrate_derSignature {
() => {
// Module: crate::der
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " ASN.1 DER-encoded signature as specified in [RFC5912 Section 6]:"] # [doc = ""] # [doc = " ```text"] # [doc = " ECDSA-Sig-Value ::= SEQUENCE {"] # [doc = "   r  INTEGER,"] # [doc = "   s  INTEGER"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC5912 Section 6]: https://www.rfc-editor.org/rfc/rfc5912#section-6"] pub struct Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { # [doc = " ASN.1 DER-encoded signature data"] bytes : SignatureBytes < C > , # [doc = " Range of the `r` value within the signature"] r_range : Range < usize > , # [doc = " Range of the `s` value within the signature"] s_range : Range < usize > , }
};
}
