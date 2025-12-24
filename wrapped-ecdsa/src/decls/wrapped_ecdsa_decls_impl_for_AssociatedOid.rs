use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// ECDSA [`ObjectIdentifier`] which identifies the digest used by default
/// with the `Signer` and `Verifier` traits.
///
/// To support non-default digest algorithms, use the [`SignatureWithOid`]
/// type instead.
#[cfg(all(feature = "digest", feature = "hazmat"))]
impl<C> AssociatedOid for Signature<C>
where
    C: hazmat::DigestAlgorithm,
    C::Digest: AssociatedOid,
{
    const OID: ObjectIdentifier = match ecdsa_oid_for_digest(C::Digest::OID) {
        Some(oid) => oid,
        None => panic!("no RFC5758 ECDSA OID defined for DigestAlgorithm::Digest"),
    };
}
