use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Structure representing the streaming context of an HKDF-Extract operation.
///
/// This type is generic over HMAC implementation. Most users should use
/// [`HkdfExtract`] or [`SimpleHkdfExtract`] type aliases.
#[derive(Clone, Debug)]
pub struct GenericHkdfExtract<H: HmacImpl> {
    hmac: H,
}
