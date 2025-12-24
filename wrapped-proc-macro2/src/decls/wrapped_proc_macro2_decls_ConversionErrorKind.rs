use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Error when retrieving a string literal's unescaped value.
#[cfg(procmacro2_semver_exempt)]
#[derive(Debug, PartialEq, Eq)]
pub enum ConversionErrorKind {
    /// The literal is of the right string kind, but its contents are malformed
    /// in a way that cannot be unescaped to a value.
    FailedToUnescape(EscapeError),
    /// The literal is not of the string kind whose value was requested, for
    /// example byte string vs UTF-8 string.
    InvalidLiteralKind,
}
