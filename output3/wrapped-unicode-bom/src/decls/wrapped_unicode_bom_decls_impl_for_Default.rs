use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for Bom {
    /// Returns the default/empty BOM type, `Bom::Null`.
    fn default() -> Self {
        Bom::Null
    }
}
