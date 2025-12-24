use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for EncodingVisitor {
    type Value = &'static Encoding;
    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a valid encoding label")
    }
    fn visit_str<E>(self, value: &str) -> Result<&'static Encoding, E>
    where
        E: serde::de::Error,
    {
        if let Some(enc) = Encoding::for_label(value.as_bytes()) {
            Ok(enc)
        } else {
            Err(E::custom(alloc::format!("invalid encoding label: {}", value)))
        }
    }
}
