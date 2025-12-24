use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// ECDSA `AlgorithmIdentifier` which identifies the digest used by default
/// with the `Signer` and `Verifier` traits.
#[cfg(feature = "pkcs8")]
impl<C> AssociatedAlgorithmIdentifier for Signature<C>
where
    C: EcdsaCurve,
    Self: AssociatedOid,
{
    type Params = AnyRef<'static>;
    const ALGORITHM_IDENTIFIER: AlgorithmIdentifierRef<'static> = AlgorithmIdentifierRef {
        oid: Self::OID,
        parameters: None,
    };
}
