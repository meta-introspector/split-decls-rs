use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Selection for overriding color output
#[allow(clippy::exhaustive_enums)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ColorChoice {
    /// Use colors if the output device appears to support them
    #[default]
    Auto,
    /// Like `Always`, except it never tries to use anything other than emitting ANSI
    /// color codes.
    AlwaysAnsi,
    /// Try very hard to emit colors.
    ///
    /// This includes emitting ANSI colors on Windows if the console API is unavailable.
    Always,
    /// Never emit colors.
    Never,
}
