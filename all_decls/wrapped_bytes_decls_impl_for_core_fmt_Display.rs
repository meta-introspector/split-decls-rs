use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl core::fmt::Display for TryGetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "Not enough bytes remaining in buffer to read value (requested {} but only {} available)",
            self.requested, self.available
        )
    }
}
