macro_rules! deps {
    () => {
        EcdsaCurve!();
        Signature!();
    };
}

macro_rules! SignatureWithOid {
    () => {
        deps!();
        # [doc = " An extended [`Signature`] type which is parameterized by an"] # [doc = " `ObjectIdentifier` which identifies the ECDSA variant used by a"] # [doc = " particular signature."] # [doc = ""] # [doc = " Valid `ObjectIdentifiers` are defined in [RFC5758 § 3.2]:"] # [doc = ""] # [doc = " - SHA-224: [`ECDSA_SHA224_OID`] (1.2.840.10045.4.3.1)"] # [doc = " - SHA-256: [`ECDSA_SHA256_OID`] (1.2.840.10045.4.3.2)"] # [doc = " - SHA-384: [`ECDSA_SHA384_OID`] (1.2.840.10045.4.3.3)"] # [doc = " - SHA-512: [`ECDSA_SHA512_OID`] (1.2.840.10045.4.3.4)"] # [doc = ""] # [doc = " [RFC5758 § 3.2]: https://www.rfc-editor.org/rfc/rfc5758#section-3.2"] # [cfg (feature = "digest")] # [derive (Clone , Eq , PartialEq)] pub struct SignatureWithOid < C : EcdsaCurve > { # [doc = " Inner signature type."] signature : Signature < C > , # [doc = " OID which identifies the ECDSA variant used."] # [doc = ""] # [doc = " MUST be one of the ECDSA algorithm variants as defined in RFC5758."] # [doc = ""] # [doc = " These OIDs begin with `1.2.840.10045.4`."] oid : ObjectIdentifier , }
    };
}

SignatureWithOid!()