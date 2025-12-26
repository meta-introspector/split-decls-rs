use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Extension trait to check whether something is a terminal.
pub trait IsTerminal {
    /// Returns true if this is a terminal.
    ///
    /// # Example
    ///
    /// ```
    /// use is_terminal::IsTerminal;
    ///
    /// if std::io::stdout().is_terminal() {
    ///     println!("stdout is a terminal")
    /// }
    /// ```
    fn is_terminal(&self) -> bool;
}
