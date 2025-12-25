use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "digest")]
impl<C> Copy for SignatureWithOid<C>
where
    C: EcdsaCurve,
    SignatureSize<C>: ArraySize,
    <SignatureSize<C> as ArraySize>::ArrayType<u8>: Copy,
{
}
