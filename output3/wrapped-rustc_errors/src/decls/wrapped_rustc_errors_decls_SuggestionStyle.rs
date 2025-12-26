use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Encodable, Decodable)]
pub enum SuggestionStyle {
    /// Hide the suggested code when displaying this suggestion inline.
    HideCodeInline,
    /// Always hide the suggested code but display the message.
    HideCodeAlways,
    /// Do not display this suggestion in the cli output, it is only meant for tools.
    CompletelyHidden,
    /// Always show the suggested code.
    /// This will *not* show the code if the suggestion is inline *and* the suggested code is
    /// empty.
    ShowCode,
    /// Always show the suggested code independently.
    ShowAlways,
}
