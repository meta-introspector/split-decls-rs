use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Structure representing the HKDF, capable of HKDF-Expand and HKDF-Extract operations.
/// Recommendations for the correct usage of the parameters can be found in the
/// [crate root](index.html#usage).
///
/// This type is generic over HMAC implementation. Most users should use
/// [`Hkdf`] or [`SimpleHkdf`] type aliases.
#[derive(Clone, Debug)]
pub struct GenericHkdf<H: HmacImpl> {
    hmac: H,
}
