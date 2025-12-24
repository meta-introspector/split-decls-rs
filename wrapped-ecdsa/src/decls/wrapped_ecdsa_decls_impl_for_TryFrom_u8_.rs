use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// NOTE: this implementation assumes the default digest for the given elliptic
/// curve as defined by [`hazmat::DigestAlgorithm`].
///
/// When working with alternative digests, you will need to use e.g.
/// [`SignatureWithOid::new_with_digest`].
#[cfg(all(feature = "digest", feature = "hazmat"))]
impl<C> TryFrom<&[u8]> for SignatureWithOid<C>
where
    C: hazmat::DigestAlgorithm,
    C::Digest: AssociatedOid,
    SignatureSize<C>: ArraySize,
{
    type Error = Error;
    fn try_from(slice: &[u8]) -> Result<Self> {
        Self::new(Signature::<C>::from_slice(slice)?, C::Digest::OID)
    }
}
