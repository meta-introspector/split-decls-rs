use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parsed challenge (scheme and body) using references to the original header value.
/// Produced by [`crate::parser::ChallengeParser`].
///
/// This is not directly useful for responding to a challenge; it's an
/// intermediary for constructing a client that knows how to respond to a specific
/// challenge scheme. In most cases, callers should construct a [`PasswordClient`]
/// without directly using `ChallengeRef`.
///
/// Only supports the param form, not the apocryphal `token68` form, as described
/// in [`crate::parser::ChallengeParser`].
#[derive(Clone, Eq, PartialEq)]
pub struct ChallengeRef<'i> {
    /// The scheme name, which should be compared case-insensitively.
    pub scheme: &'i str,
    /// Zero or more parameters.
    ///
    /// These are represented as a `Vec` of key-value pairs rather than a
    /// map. Given that the parameters are generally only used once when
    /// constructing a challenge client and each challenge only supports a few
    /// parameter types, it's more efficient in terms of CPU usage and code size
    /// to scan through them directly.
    pub params: Vec<ChallengeParamRef<'i>>,
}
