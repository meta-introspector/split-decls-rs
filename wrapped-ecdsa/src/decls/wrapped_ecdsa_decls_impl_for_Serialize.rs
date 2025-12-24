use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "serde")]
impl<C> Serialize for Signature<C>
where
    C: EcdsaCurve,
    SignatureSize<C>: ArraySize,
{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serdect::array::serialize_hex_upper_or_bin(&self.to_bytes(), serializer)
    }
}
