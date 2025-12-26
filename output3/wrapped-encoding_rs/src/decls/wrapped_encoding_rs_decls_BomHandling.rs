use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Communicate the BOM handling mode.
#[derive(Debug, Copy, Clone)]
enum BomHandling {
    /// Don't handle the BOM
    Off,
    /// Sniff for UTF-8, UTF-16BE or UTF-16LE BOM
    Sniff,
    /// Remove the BOM only if it's the BOM for this encoding
    Remove,
}
