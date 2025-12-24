use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns `true` if `this` is a terminal.
///
/// This is equivalent to calling `this.is_terminal()` and exists only as a
/// convenience to calling the trait method [`IsTerminal::is_terminal`]
/// without importing the trait.
///
/// # Example
///
/// ```
/// if is_terminal::is_terminal(&std::io::stdout()) {
///     println!("stdout is a terminal")
/// }
/// ```
pub fn is_terminal<T: IsTerminal>(this: T) -> bool {
    this.is_terminal()
}
