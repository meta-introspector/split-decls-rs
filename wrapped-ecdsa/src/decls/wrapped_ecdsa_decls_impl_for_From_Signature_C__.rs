use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<C> From<Signature<C>> for SignatureBytes<C>
where
    C: EcdsaCurve,
    SignatureSize<C>: ArraySize,
{
    fn from(signature: Signature<C>) -> SignatureBytes<C> {
        signature.to_bytes()
    }
}
