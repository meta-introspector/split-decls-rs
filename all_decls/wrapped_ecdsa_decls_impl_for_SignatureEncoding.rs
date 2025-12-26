use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// NOTE: this implementation assumes the default digest for the given elliptic
/// curve as defined by [`hazmat::DigestAlgorithm`].
///
/// When working with alternative digests, you will need to use e.g.
/// [`SignatureWithOid::new_with_digest`].
#[cfg(all(feature = "digest", feature = "hazmat"))]
impl<C> SignatureEncoding for SignatureWithOid<C>
where
    C: hazmat::DigestAlgorithm,
    C::Digest: AssociatedOid,
    SignatureSize<C>: ArraySize,
{
    type Repr = SignatureBytes<C>;
}
