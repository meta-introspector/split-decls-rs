use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl EscapeError {
    /// Returns true for actual errors, as opposed to warnings.
    pub fn is_fatal(&self) -> bool {
        !matches!(
            self,
            EscapeError::UnskippedWhitespaceWarning | EscapeError::MultipleSkippedLinesWarning
        )
    }
}
