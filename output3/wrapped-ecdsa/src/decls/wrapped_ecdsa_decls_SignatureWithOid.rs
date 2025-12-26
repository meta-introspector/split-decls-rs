use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An extended [`Signature`] type which is parameterized by an
/// `ObjectIdentifier` which identifies the ECDSA variant used by a
/// particular signature.
///
/// Valid `ObjectIdentifiers` are defined in [RFC5758 § 3.2]:
///
/// - SHA-224: [`ECDSA_SHA224_OID`] (1.2.840.10045.4.3.1)
/// - SHA-256: [`ECDSA_SHA256_OID`] (1.2.840.10045.4.3.2)
/// - SHA-384: [`ECDSA_SHA384_OID`] (1.2.840.10045.4.3.3)
/// - SHA-512: [`ECDSA_SHA512_OID`] (1.2.840.10045.4.3.4)
///
/// [RFC5758 § 3.2]: https://www.rfc-editor.org/rfc/rfc5758#section-3.2
#[cfg(feature = "digest")]
#[derive(Clone, Eq, PartialEq)]
pub struct SignatureWithOid<C: EcdsaCurve> {
    /// Inner signature type.
    signature: Signature<C>,
    /// OID which identifies the ECDSA variant used.
    ///
    /// MUST be one of the ECDSA algorithm variants as defined in RFC5758.
    ///
    /// These OIDs begin with `1.2.840.10045.4`.
    oid: ObjectIdentifier,
}
